use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::slice::StaticChunkable;

#[derive(Serialize, Deserialize)]
pub struct Embedding(String); // base64

impl Embedding {
    pub fn to_vec(&self) -> anyhow::Result<Vec<f32>> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(&self.0)?;
        let chunks = bytes
            .as_slice()
            .static_chunks()
            .ok_or(anyhow::anyhow!("Embedding cannot be split into f32 values"))?;
        Ok(chunks.map(|chunk| f32::from_le_bytes(*chunk)).collect())
    }
}

#[derive(Serialize)]
struct EmbedRequest {
    input: Vec<String>,
    model: String,
    encoding_format: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    data: Option<Vec<EmbedResponseData>>,
    error: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct EmbedResponseData {
    embedding: String,
}

pub fn embed<S0, S1>(inputs: Vec<String>, model: S0, key: S1) -> anyhow::Result<Vec<Embedding>>
where
    S0: ToString,
    S1: AsRef<str>,
{
    let body = EmbedRequest {
        input: inputs,
        model: model.to_string(),
        encoding_format: "base64".to_string(),
    };

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.openai.com/v1/embeddings")
        .header("Authorization", format!("Bearer {}", key.as_ref()))
        .json(&body)
        .send()?;

    let response: EmbedResponse = response.json()?;

    if let Some(data) = response.data {
        return Ok(data.into_iter().map(|e| Embedding(e.embedding)).collect());
    } else if let Some(data) = response.error {
        anyhow::bail!("Error in completion response: {:?}", data);
    } else {
        anyhow::bail!("No data in response");
    }
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    let dot = a
        .iter()
        .zip(b)
        .map(|(a, b)| *a as f64 * *b as f64)
        .reduce(|sum, x| sum + x)
        .unwrap_or(0.0);
    let a_len = a
        .iter()
        .map(|a| *a as f64 * *a as f64)
        .reduce(|sum, a| sum + a)
        .unwrap_or(0.0)
        .sqrt();
    let b_len = b
        .iter()
        .map(|b| *b as f64 * *b as f64)
        .reduce(|sum, b| sum + b)
        .unwrap_or(0.0)
        .sqrt();
    dot / (a_len * b_len)
}
