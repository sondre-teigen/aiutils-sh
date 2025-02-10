use std::{
    collections::VecDeque,
    io::{BufRead as _, BufReader, Write as _},
};

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

        let mut total_lines = 0;
        let mut head = Vec::new();
        let mut tail = VecDeque::new();
        for line in BufReader::new(stdout).lines() {
            let line = line?;
            total_lines += 1;
            if args.head.is_none() && args.tail.is_none() {
                writeln!(out, "{}", line)?;
            } else {
                if let Some(count) = args.head {
                    if head.len() < count {
                        head.push(line.clone());
                    }
                }
                if let Some(count) = args.tail {
                    while tail.len() >= count {
                        tail.pop_front();
                    }
                    tail.push_back(line);
                }
            }
        }

        if args.head.is_some() || args.tail.is_some() {
            let overlap = (head.len() + tail.len()).saturating_sub(total_lines);

            for _ in 0..overlap {
                tail.pop_front();
            }

            let omitted = total_lines.saturating_sub(head.len() + tail.len());
            for line in head {
                writeln!(out, "{}", line)?;
            }
            if omitted > 0 {
                writeln!(out, "[... {} lines omitted]", omitted)?;
            }
            for line in tail {
                writeln!(out, "{}", line)?;
            }
        }

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
