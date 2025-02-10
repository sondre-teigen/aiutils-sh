use std::path::PathBuf;

use aituils_sh::api::{Message, Role};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "-")]
    content: PathBuf,
    #[arg(long, default_value = "user")]
    role: Role,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let content = aituils_sh::fs::read_string(args.content)?;

    let message = vec![Message {
        role: args.role,
        content: content,
    }];

    serde_json::to_writer(&mut std::io::stdout(), &message)?;

    Ok(())
}
