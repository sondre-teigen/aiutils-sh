use std::{
    io::{BufRead as _, Write},
    path::PathBuf,
};

use clap::Parser;

/// Parse and extract markdown code blocks
#[derive(Parser)]
struct Cli {
    /// Input file. Use - to read stdin
    #[arg(default_value = "-")]
    file: PathBuf,

    /// Redirect non-codeblock text
    #[arg(long)]
    redirect_rest: Option<PathBuf>,

    /// Extract specific code block
    #[arg(long)]
    block_index: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let file = aituils_sh::fs::open_buffered(args.file.as_path())?;

    let mut block_out = aituils_sh::fs::create(args.file)?;
    let mut rest_out = match args.redirect_rest {
        Some(path) => Some(aituils_sh::fs::create(path)?),
        None => None,
    };

    let mut in_block = false;
    let mut block_index = 0;
    for line in file.lines() {
        let line = line?;
        if line.starts_with("```") {
            if !in_block {
                in_block = true;
                continue;
            } else {
                in_block = false;
                block_index += 1;
            }
        } else {
            if in_block
                && (args
                    .block_index
                    .and_then(|i| Some(i == block_index))
                    .unwrap_or(true))
            {
                writeln!(block_out, "{}", line)?;
            } else if let Some(ref mut rest_out) = rest_out {
                writeln!(rest_out, "{}", line)?;
            }
        }
    }

    Ok(())
}
