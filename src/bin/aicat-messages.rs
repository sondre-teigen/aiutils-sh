use std::path::PathBuf;

use aituils_sh::api::Message;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(default_value = "-")]
    messages: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut all_messages: Vec<Message> = vec![];
    for path in args.messages {
        all_messages.append(&mut aituils_sh::fs::read_json(path)?);
    }

    aituils_sh::fs::print_json(&all_messages)?;

    Ok(())
}
