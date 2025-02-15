use std::{collections::HashMap, io::Write as _, path::PathBuf};

use clap::Parser;

/// Combine embeddings into a single JSON object
#[derive(Parser)]
struct Cli {
    /// Files to combine. Use - to read stdin
    files: Vec<PathBuf>,
}

type Embeddings = HashMap<String, aituils_sh::api::Embedding>;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let mut embeddings = HashMap::new();
    for embedding in args.files {
        embeddings =
            serde_json::from_reader::<_, Embeddings>(aituils_sh::fs::open_buffered(embedding)?)?
                .into_iter()
                .fold(embeddings, |mut embeddings, (key, embedding)| {
                    embeddings.insert(key, embedding);
                    embeddings
                });
    }

    serde_json::to_writer(std::io::stdout(), &embeddings)?;
    writeln!(std::io::stdout())?;

    Ok(())
}
