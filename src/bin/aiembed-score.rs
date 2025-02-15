use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    io::BufReader,
    path::PathBuf,
};

use clap::Parser;

/// Compare and sort embedding similarities
#[derive(Parser)]
struct Cli {
    /// Query embeddings
    #[arg(default_value = "-")]
    query: PathBuf,

    /// Embeddings to compare query with 
    embeddings: Vec<PathBuf>,

    /// Max result limit
    #[arg(long)]
    limit: Option<usize>,
    /// Similarity threshold
    #[arg(long)]
    threshold: Option<f64>,
    /// Print scores in addition to result names
    #[arg(long)]
    score: bool,
}

#[derive(PartialEq, PartialOrd)]
struct Double(f64);

impl Eq for Double {}

impl Ord for Double {
    fn cmp(&self, other: &Self) -> Ordering {
        let diff = self.0 - other.0;
        if diff < 0.0 {
            Ordering::Less
        } else if diff > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Score {
    score: Double,
    path: String,
}

type Embeddings = HashMap<String, aituils_sh::api::Embedding>;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let query: Embeddings = serde_json::from_reader(aituils_sh::fs::open_buffered(args.query)?)?;
    let query = query.into_values().try_fold(vec![], |mut vec, embedding| {
        vec.push(embedding.to_vec()?);
        anyhow::Result::<_, anyhow::Error>::Ok(vec)
    })?;

    let mut embeddings = HashMap::new();
    for embedding in args.embeddings {
        embeddings = serde_json::from_reader::<_, Embeddings>(BufReader::new(
            std::fs::File::open(embedding)?,
        ))?
        .into_iter()
        .fold(embeddings, |mut embeddings, (key, embedding)| {
            embeddings.insert(key, embedding);
            embeddings
        });
    }

    let mut scores: BinaryHeap<Score> = BinaryHeap::new();

    for (key, embedding) in embeddings {
        let embedding = embedding.to_vec()?;
        let mut max_similarity = 0.0;
        for q in &query {
            let similarity = aituils_sh::api::cosine_similarity(&embedding, q);
            if max_similarity < similarity {
                max_similarity = similarity;
            }
        }
        scores.push(Score {
            score: Double(max_similarity),
            path: key,
        });
    }

    let limit = args.limit.unwrap_or(scores.len());
    for _ in 0..limit {
        let Some(score) = scores.pop() else {
            break;
        };
        if let Some(threshold) = args.threshold {
            if score.score.0 < threshold {
                break;
            }
        }
        if args.score {
            println!("{} {}", score.score.0, score.path);
        } else {
            println!("{}", score.path);
        }
    }

    Ok(())
}
