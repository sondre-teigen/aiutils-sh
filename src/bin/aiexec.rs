use std::io::{BufReader, Write};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,

    #[arg(long)]
    forward_status: bool,
    #[arg(long)]
    no_stderr: bool,

    #[arg(long)]
    head: Option<usize>,
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
        writeln!(out, "")?;

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
