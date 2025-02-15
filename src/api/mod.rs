mod complete;
pub use complete::{complete, Message, Role};

mod embed;
pub use embed::{embed, Embedding, cosine_similarity};

pub fn get_key() -> anyhow::Result<String> {
    Ok(std::env::var("OPENAI_API_KEY")?)
}
