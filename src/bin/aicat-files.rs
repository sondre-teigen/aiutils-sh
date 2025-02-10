use std::path::PathBuf;

use clap::Parser;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "-")]
    files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let out = &mut std::io::stdout();

    for (i, file) in args.files.iter().enumerate() {
        if i != 0 {
            writeln!(out, "")?;
        }
        aituils_sh::fs::cat(out, file.as_path())?;
    }

    Ok(())
}
