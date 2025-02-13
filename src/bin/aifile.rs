use std::path::PathBuf;

use clap::Parser;
use std::io::Write as _;

#[derive(Parser)]
struct Cli {
    file: PathBuf,
    #[arg(long)]
    name: Option<PathBuf>,
    #[arg(long)]
    language: Option<String>,
    #[arg(long)]
    head: Option<usize>,
    #[arg(long)]
    tail: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let out = &mut std::io::stdout().lock();

    let name = args.name.as_ref().unwrap_or(&args.file);
    let language = args
        .language
        .as_ref()
        .and_then(|s| Some(s.as_str()))
        .unwrap_or("");
    
    writeln!(out, "")?;
    writeln!(out, "File: `{}`", name.display())?;
    writeln!(out, "```{}", language)?;
    aituils_sh::io::write_lines_partial(
        out,
        aituils_sh::fs::open_buffered(args.file.as_path())?,
        args.head,
        args.tail,
    )?;
    writeln!(out, "```")?;

    Ok(())
}
