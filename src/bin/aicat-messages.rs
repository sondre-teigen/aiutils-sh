use std::path::PathBuf;

use aituils_sh::api::Message;
use clap::Parser;

/// Collect and splice API markup messages
#[derive(Parser)]
struct Cli {
    /// Files containing API markup messages. Use - to read stdin
    #[arg(default_value = "-")]
    files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut all_messages: Vec<Message> = vec![];
    for path in args.files {
        all_messages.append(&mut aituils_sh::fs::read_json(path)?);
    }

    serde_json::to_writer(&mut std::io::stdout(), &all_messages)?;

    Ok(())
}
