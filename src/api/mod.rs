mod complete;
pub use complete::{complete, Message, Role};

mod embed;
pub use embed::{cosine_similarity, embed, Embedding};

pub fn get_key() -> anyhow::Result<String> {
    Ok(std::env::var("OPENAI_API_KEY")?)
}
