use anyhow::Result;
use clap::Parser;
use std::io::{self, IsTerminal, Read, Write};
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(name = "gi")]
#[command(about = "A linter/formatter wrapper for `jj fix`")]
#[command(long_about = "gi: A linter/formatter wrapper for `jj fix`")]
struct Args {
    /// The command and arguments to execute
    #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,
}

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
        let handle = std::thread::spawn(move || -> io::Result<()> {
            stdin.write_all(&stdin_content)?;
            Ok(())
        });
        match handle.join() {
            Ok(result) => result?,
            Err(_) => anyhow::bail!("failed to write to child's stdin"),
        }
    }

    let output = child.wait_with_output()?;

    // Write to stdout.
    io::stdout().write_all(&output.stdout)?;

    // Exit with the same status code as the child process.
    std::process::exit(output.status.code().unwrap_or(1));
}
