use std::collections::VecDeque;
use std::io::{BufRead, Write};
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

    #[arg(long)]
    head: Option<usize>,
    #[arg(long)]
    tail: Option<usize>,
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

        if let Some(stdout) = child.stdout.take().map(|r| BufReader::new(r)) {
            let preface = format!("Command `{}` stdout", args.command.join(" "));
            if let Some(head) = args.head {
                cat_head(out, stdout, preface, head)?;
            } else if let Some(tail) = args.tail {
                cat_tail(out, stdout, preface, tail)?;
            } else {
                cat_all(out, stdout, preface)?;
            }
        };

        if !args.no_stderr {
            if let Some(stderr) = child.stderr.take().map(|r| BufReader::new(r)) {
                let preface = format!("Command `{}` stderr", args.command.join(" "));
                if let Some(head) = args.head {
                    cat_head(out, stderr, preface, head)?;
                } else if let Some(tail) = args.tail {
                    cat_tail(out, stderr, preface, tail)?;
                } else {
                    cat_all(out, stderr, preface)?;
                };
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

fn cat_all<W, R, S>(out: &mut W, reader: R, preface: S) -> anyhow::Result<bool>
where
    W: Write,
    R: BufRead,
    S: AsRef<str>,
{
    Ok(cat_iterator(out, reader.lines(), preface.as_ref())?)
}

fn cat_head<W, R, S>(out: &mut W, reader: R, preface: S, count: usize) -> anyhow::Result<bool>
where
    W: Write,
    R: BufRead,
    S: AsRef<str>,
{
    let mut buffer = Vec::new();
    let mut i = 0;
    for line in reader.lines() {
        if buffer.len() < count {
            buffer.push(line);
        }
        i += 1;
    }
    if i > count {
        buffer.push(Ok(format!("[... {} lines omitted]", i - count)));
    }
    Ok(cat_iterator(out, buffer.into_iter(), preface.as_ref())?)
}

fn cat_tail<W, R, S>(out: &mut W, reader: R, preface: S, count: usize) -> anyhow::Result<bool>
where
    W: Write,
    R: BufRead,
    S: AsRef<str>,
{
    let mut buffer = VecDeque::new();
    let mut i = 0;
    for line in reader.lines() {
        while buffer.len() >= count {
            buffer.pop_front();
        }
        buffer.push_back(line);
        i += 1;
    }
    if i > count {
        buffer.push_front(Ok(format!("[... {} lines omitted]", i - count)));
    }
    Ok(cat_iterator(out, buffer.into_iter(), preface.as_ref())?)
}

fn cat_iterator<W, I, S>(out: &mut W, lines: I, preface: S) -> anyhow::Result<bool>
where
    W: Write,
    I: IntoIterator<Item = std::io::Result<String>>,
    S: AsRef<str>,
{
    let mut empty = true;
    for line in lines {
        if empty {
            writeln!(out, "{}:", preface.as_ref())?;
            writeln!(out, "```")?;
            empty = false;
        }
        writeln!(out, "{}", line?)?;
    }
    if !empty {
        writeln!(out, "```")?;
    }
    Ok(!empty)
}
