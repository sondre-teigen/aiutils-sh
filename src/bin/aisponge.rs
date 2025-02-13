use std::{
    io::{BufRead as _, BufReader, Seek, Write as _},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
struct Cli {
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(args.output)?;
    file.seek(std::io::SeekFrom::Start(0))?;

    aituils_sh::io::write_lines(&mut file, BufReader::new(std::io::stdin()))?;

    let position = file.stream_position()?;
    file.set_len(position)?;

    Ok(())
}
