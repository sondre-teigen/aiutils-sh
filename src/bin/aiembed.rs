use std::{collections::HashMap, io::Write as _, path::PathBuf};

use clap::Parser;

/// Generate embeddings of the contents of the input files
#[derive(Parser)]
struct Cli {
    /// Files to embed. Use - to read stdin. Specify <name>:<path> to rename files in the output.
    files: Vec<String>,

    /// File to store embeddings in JSON. Use - to write to stdout
    #[arg(long, default_value = "-")]
    output: PathBuf,

    /// Model to use for generating embeddings
    #[arg(long, default_value = "text-embedding-3-small")]
    model: String,

    /// Number of files to send per embedding request
    #[arg(long, default_value = "4")]
    chunk: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let key = aituils_sh::api::get_key()?;

    let mut embeddings = HashMap::new();

    for files in args.files.chunks(args.chunk) {
        let files: Vec<_> = files
            .iter()
            .map(|file| file.split_once(":").unwrap_or((&file, &file)))
            .collect();

        let mut inputs = vec![];
        for (_, file) in files.iter() {
            inputs.push(aituils_sh::fs::read_string(file)?);
        }
        let outputs = aituils_sh::api::embed(inputs, &args.model, &key)?;
        for ((name, _), embedding) in files.into_iter().zip(outputs) {
            embeddings.insert(name.to_string(), embedding);
        }
    }

    serde_json::to_writer(std::io::stdout(), &embeddings)?;
    writeln!(std::io::stdout())?;
    Ok(())
}
