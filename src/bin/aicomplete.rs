use std::path::PathBuf;

use aituils_sh::api::Message;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "-")]
    messages: PathBuf,

    #[arg(long)]
    prediction: Option<PathBuf>,

    #[arg(long)]
    stream: bool,
    #[arg(long, default_value = "gpt-4o-mini")]
    model: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let messages: Vec<Message> = aituils_sh::fs::read_json(args.messages)?;
    let prediction = match args.prediction {
        Some(path) => Some(aituils_sh::fs::read_string(path)?),
        None => None,
    };

    let key = aituils_sh::api::get_key()?;

    aituils_sh::api::complete(messages, args.model, prediction, key, args.stream)?;

    Ok(())
}
