use std::io::{BufRead as _, Write as _};
use std::{
    io::BufReader,
    process::{Command, Stdio},
};

use anyhow::Context as _;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    command: Vec<String>,

    #[arg(long)]
    forward_status: bool,
    #[arg(long)]
    no_stderr: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let status = {
        let out = &mut std::io::stdout().lock();

        anyhow::ensure!(!args.command.is_empty(), "No command provided");

        let mut child = Command::new(&args.command[0])
            .args(&args.command[1..])
            .stdout(Stdio::piped())
            .stderr(if args.no_stderr {
                Stdio::inherit()
            } else {
                Stdio::piped()
            })
            .spawn()
            .with_context(|| format!("Failed to execute command: {}", args.command.join(" ")))?;

        let Some(stdout) = child.stdout.take() else {
            anyhow::bail!("Failed to capture stdout");
        };
        let mut stdout_empty = true;

        let stderr = child.stderr.take();
        let mut stderr_empty = true;

        {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if stdout_empty {
                    writeln!(out, "Command `{}` stdout:", args.command.join(" "))?;
                    writeln!(out, "```")?;
                    stdout_empty = false;
                }
                writeln!(out, "{}", line?)?;
            }
            if !stdout_empty {
                writeln!(out, "```")?;
            }
        }

        if let Some(stderr) = stderr {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if stderr_empty {
                    writeln!(out, "Command `{}` stderr:", args.command.join(" "))?;
                    writeln!(out, "```")?;
                    stderr_empty = false;
                }
                writeln!(out, "{}", line?)?;
            }
            if !stderr_empty {
                writeln!(out, "```")?;
            }
        }

        anyhow::Result::<_, anyhow::Error>::Ok(child.wait()?)
    }?;

    if args.forward_status {
        std::process::exit(status.code().unwrap_or(0));
    } else {
        Ok(())
    }
}
