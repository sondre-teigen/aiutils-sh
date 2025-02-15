use std::io::{stdout, BufRead as _, BufReader, Write as _};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, ValueEnum, Clone, Copy, Debug)]
pub enum Role {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Debug)]
struct CompletionRequest {
    messages: Vec<Message>,
    model: String,
    prediction: Option<Prediction>,
    stream: bool,
}

#[derive(Serialize, Debug)]
struct Prediction {
    r#type: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct CompletionResponse {
    choices: Option<Vec<CompletionResponseChoice>>,
    error: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct CompletionResponseChoice {
    message: CompletionResponseMessage,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
struct CompletionResponseMessage {
    role: Role,
    content: String,
}

#[derive(Deserialize, Debug)]
struct CompletionStreamResponse {
    choices: Option<Vec<CompletionStreamResponseChoice>>,
    error: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct CompletionStreamResponseChoice {
    delta: CompletionStreamResponseDelta,
}

#[derive(Deserialize, Debug)]
struct CompletionStreamResponseDelta {
    content: Option<String>,
}

pub fn complete<S0, S1, S2>(
    messages: Vec<Message>,
    model: S0,
    prediction: Option<S1>,
    key: S2,
    stream: bool,
) -> anyhow::Result<()>
where
    S0: ToString,
    S1: ToString,
    S2: AsRef<str>,
{
    let body = CompletionRequest {
        messages,
        model: model.to_string(),
        prediction: prediction.map(|s| Prediction {
            r#type: "content".to_string(),
            content: s.to_string(),
        }),
        stream,
    };

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", key.as_ref()))
        .header("Accept", "text/event-stream")
        .json(&body)
        .send()?;

    if stream {
        for response in BufReader::new(response).lines() {
            let response = response?;
            if let Some(data) = response.strip_prefix("data: ").map(|data| data.trim()) {
                if data.is_empty() {
                    continue;
                }

                if data == "[DONE]" {
                    break;
                }

                let data: CompletionStreamResponse = serde_json::from_str(data)?;
                if let Some(choices) = data.choices {
                    if let Some(choice) = choices.get(0) {
                        if let Some(content) = &choice.delta.content {
                            write!(stdout(), "{}", content)?;
                            stdout().flush()?;
                        } else {
                            writeln!(stdout())?;
                        }
                    }
                } else if let Some(data) = data.error {
                    anyhow::bail!("Error in completion response: {:?}", data);
                } else {
                    std::io::stderr().write("No data in response\n".as_bytes())?;
                }
            }
        }
    } else {
        let response: CompletionResponse = response.json()?;

        if let Some(choices) = response.choices {
            if let Some(choice) = choices.get(0) {
                writeln!(stdout(), "{}", choice.message.content)?;
            }
        } else if let Some(data) = response.error {
            anyhow::bail!("Error in completion response: {:?}", data);
        } else {
            std::io::stderr().write("No data in response\n".as_bytes())?;
        }
    }
    Ok(())
}
