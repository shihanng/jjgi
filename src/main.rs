use anyhow::Result;
use clap::Parser;
use std::io::{self, IsTerminal, Read, Write};
use std::process::{Command, Stdio};

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputSource {
    /// Use the stdout from the command
    Stdout,
    /// Use the stdin
    Stdin,
    /// Use the stderr from the command
    Stderr,
    /// Use the content from '{stdin_file}'
    StdinFile,
    /// Use the content from '{file}'
    File,
}

#[derive(Parser, Debug)]
#[command(name = "jjgi")]
#[command(about = "A linter/formatter wrapper for `jj fix`")]
#[command(long_about = "jjgi: A linter/formatter wrapper for `jj fix`")]
struct Args {
    /// The command and arguments to execute
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,
    /// Content of jjgi's stdout when the command executes successfully
    #[arg(long, value_enum, default_value_t=OutputSource::Stdout)]
    on_success_stdout: OutputSource,
    /// Content of jjgi's stderr when the command executes successfully
    #[arg(long, value_enum, default_value_t=OutputSource::Stderr)]
    on_success_stderr: OutputSource,
    /// Store stdin in a temporary file that the command can access by referencing '{stdin_file}'
    #[arg(long, default_value_t = false, conflicts_with = "file")]
    stdin_file: bool,
    /// Path to a file that can be referenced using '{file}' in the command. When set, stdin will
    /// not be piped to the command.
    #[arg(long, conflicts_with = "stdin_file")]
    file: Option<String>,
}

const ERROR_CODE: i32 = 1;
const STDIN_FILE_PLACEHOLDER: &str = "{stdin_file}";
const FILE_PLACEHOLDER: &str = "{file}";

fn main() -> Result<()> {
    let args = Args::parse();

    // Read stdin only if it's piped (not a terminal).
    // TODO(shihanng): Use a flag to control this behavior in the future.
    let mut stdin_content = Vec::new();
    let is_stdin_piped = !io::stdin().is_terminal();
    if is_stdin_piped {
        io::stdin().read_to_end(&mut stdin_content)?;
    }

    // Save stdin to a temporary file if the flag is set
    let stdin_file: Option<tempfile::NamedTempFile> = if args.stdin_file {
        let mut temp = tempfile::NamedTempFile::new()?;
        temp.write_all(&stdin_content)?;
        temp.flush()?;
        Some(temp)
    } else {
        None
    };

    let stdin_file_path = if let Some(ref f) = stdin_file {
        f.path().to_str().unwrap_or(STDIN_FILE_PLACEHOLDER)
    } else {
        STDIN_FILE_PLACEHOLDER
    };

    let file_path = if let Some(ref f) = args.file {
        f
    } else {
        FILE_PLACEHOLDER
    };

    let command_args: Vec<String> = args.command[1..]
        .iter()
        .map(|arg| {
            arg.replace(STDIN_FILE_PLACEHOLDER, stdin_file_path)
                .replace(FILE_PLACEHOLDER, file_path)
        })
        .collect();

    let mut child = Command::new(&args.command[0])
        .args(&command_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write stdin content to the child process if available.
    if !args.stdin_file
        && args.file.is_none()
        && let Some(mut stdin) = child.stdin.take()
    {
        std::thread::scope(|s| {
            s.spawn(|| -> io::Result<()> {
                stdin.write_all(&stdin_content)?;
                Ok(())
            });
        });
    }

    let output = child.wait_with_output()?;
    let exit_code = output.status.code().unwrap_or(ERROR_CODE);
    match exit_code {
        0 => {
            match args.on_success_stdout {
                OutputSource::Stdout => {
                    io::stdout().write_all(&output.stdout)?;
                }
                OutputSource::Stdin => {
                    io::stdout().write_all(&stdin_content)?;
                }
                OutputSource::Stderr => {
                    io::stdout().write_all(&output.stderr)?;
                }
                OutputSource::StdinFile => {
                    if let Some(ref f) = stdin_file {
                        let content = std::fs::read(f.path())?;
                        io::stdout().write_all(&content)?;
                    }
                }
                OutputSource::File => {
                    if let Some(ref file_path) = args.file {
                        let content = std::fs::read(file_path)?;
                        io::stdout().write_all(&content)?;
                    }
                }
            }
            match args.on_success_stderr {
                OutputSource::Stdout => {
                    io::stderr().write_all(&output.stdout)?;
                }
                OutputSource::Stdin => {
                    io::stderr().write_all(&stdin_content)?;
                }
                OutputSource::Stderr => {
                    io::stderr().write_all(&output.stderr)?;
                }
                OutputSource::StdinFile => {
                    if let Some(ref f) = stdin_file {
                        let content = std::fs::read(f.path())?;
                        io::stderr().write_all(&content)?;
                    }
                }
                OutputSource::File => {
                    if let Some(ref file_path) = args.file {
                        let content = std::fs::read(file_path)?;
                        io::stderr().write_all(&content)?;
                    }
                }
            }
            std::process::exit(exit_code);
        }
        _ => {
            std::process::exit(exit_code);
        }
    }
}
