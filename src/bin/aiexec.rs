use std::io::{BufReader, Write};

use clap::Parser;

/// Execute command and format output with markup annotation
#[derive(Parser)]
struct Cli {
    /// Command to execute and capture
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,

    /// Return exit status from executed command
    #[arg(long)]
    forward_status: bool,

    /// Exclude stderr output from annotated output
    #[arg(long)]
    no_stderr: bool,

    /// Number of lines to keep from the start of the output
    #[arg(long)]
    head: Option<usize>,
    /// Number of lines to keep from the end of the output
    #[arg(long)]
    tail: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let status = {
        anyhow::ensure!(!args.command.is_empty(), "No command provided");
        let out = &mut std::io::stdout();

        let formatted_command = args
            .command
            .iter()
            .map(|part| {
                if part.is_empty() {
                    "\"\"".to_string()
                } else if part.contains(' ') {
                    format!("\"{}\"", part)
                } else {
                    part.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        
        writeln!(out, "")?;
        writeln!(out, "Command: `{}`", formatted_command)?;
        writeln!(out, "```console")?;

        let mut process = subprocess::Exec::cmd(&args.command[0])
            .args(&args.command[1..])
            .stdout(subprocess::Redirection::Pipe)
            .stderr(if args.no_stderr {
                subprocess::Redirection::None
            } else {
                subprocess::Redirection::Merge
            })
            .popen()?;

        let Some(stdout) = process.stdout.take() else {
            anyhow::bail!("No command output file");
        };

        aituils_sh::io::write_lines_partial(out, BufReader::new(stdout), args.head, args.tail)?;

        let status = process.wait()?;

        writeln!(out, "```")?;

        anyhow::Result::<_, anyhow::Error>::Ok(status)
    }?;

    if args.forward_status {
        let code = if let subprocess::ExitStatus::Exited(status) = status {
            status as i32
        } else {
            1
        };
        std::process::exit(code)
    } else {
        Ok(())
    }
}
