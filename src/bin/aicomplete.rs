use std::path::PathBuf;

use aituils_sh::api::Message;
use clap::Parser;

/// Generate a complection from API markup messages
#[derive(Parser)]
struct Cli {
    /// File containing API markup input messages. Use - to read stdin
    #[arg(default_value = "-")]
    file: PathBuf,

    /// Output prediction
    #[arg(long)]
    prediction: Option<PathBuf>,

    /// Stream output text
    #[arg(long)]
    stream: bool,

    /// AI model to use
    #[arg(long, default_value = "gpt-4o-mini")]
    model: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let messages: Vec<Message> = aituils_sh::fs::read_json(args.file)?;
    let prediction = match args.prediction {
        Some(path) => Some(aituils_sh::fs::read_string(path)?),
        None => None,
    };

    let key = aituils_sh::api::get_key()?;

    aituils_sh::api::complete(messages, args.model, prediction, key, args.stream)?;

    Ok(())
}
