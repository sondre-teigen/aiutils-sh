use std::path::PathBuf;

use aituils_sh::api::{Message, Role};
use clap::Parser;

/// Convert input text into API markup message
#[derive(Parser)]
struct Cli {
    /// Input file. Use - to read stdin.
    #[arg(default_value = "-")]
    file: PathBuf,
    /// Message role
    #[arg(long, default_value = "user")]
    role: Role,
    /// Generate an empty message markup
    #[arg(long)]
    empty: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    if args.empty {
        println!("[]");
        return Ok(());
    }

    let content = aituils_sh::fs::read_string(args.file)?;

    let message = vec![Message {
        role: args.role,
        content: content,
    }];

    serde_json::to_writer(&mut std::io::stdout(), &message)?;

    Ok(())
}
