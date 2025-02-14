use std::{
    io::{BufReader, Seek},
    path::PathBuf,
};

use clap::Parser;

/// Collect stdin into a file line by line without truncating.
#[derive(Parser)]
struct Cli {
    /// File to collect input into
    file: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(args.file)?;
    file.seek(std::io::SeekFrom::Start(0))?;

    aituils_sh::io::write_lines(&mut file, BufReader::new(std::io::stdin()))?;

    let position = file.stream_position()?;
    file.set_len(position)?;

    Ok(())
}
