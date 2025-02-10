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

    writeln!(out, "{}:", name.display())?;
    writeln!(out, "```{}", language)?;
    aituils_sh::fs::cat(out, args.file.as_path())?;
    writeln!(out, "```")?;

    Ok(())
}
