use anyhow::Result;
use clap::Parser;
use std::io::{self, IsTerminal, Read, Write};
use std::process::{Command, Stdio};

#[derive(clap::ValueEnum, Clone, Debug)]
enum StdChoice {
    /// Use the stdout from the command
    StdOut,
}

#[derive(Parser, Debug)]
#[command(name = "gi")]
#[command(about = "A linter/formatter wrapper for `jj fix`")]
#[command(long_about = "gi: A linter/formatter wrapper for `jj fix`")]
struct Args {
    /// The command and arguments to execute
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,
    /// Content of gi's stdout when the command executes successfully
    #[arg(long, value_enum, default_value_t=StdChoice::StdOut)]
    on_success_stdout: StdChoice,
}

const ERROR_CODE: i32 = 1;

fn main() -> Result<()> {
    let args = Args::parse();

    // Read stdin only if it's piped (not a terminal).
    // TODO(shihanng): Use a flag to control this behavior in the future.
    let mut stdin_content = Vec::new();
    let is_stdin_piped = !io::stdin().is_terminal();
    if is_stdin_piped {
        io::stdin().read_to_end(&mut stdin_content)?;
    }

    let mut child = Command::new(&args.command[0])
        .args(&args.command[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write stdin content to the child process if available.
    if let Some(mut stdin) = child.stdin.take() {
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
                StdChoice::StdOut => {
                    io::stdout().write_all(&output.stdout)?;
                }
            }
            std::process::exit(exit_code);
        }
        _ => {
            std::process::exit(exit_code);
        }
    }
}
