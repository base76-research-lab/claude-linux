use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatResponse {
    content: Vec<ChatContent>,
}

#[derive(Deserialize)]
struct ChatContent {
    #[serde(rename = "type")]
    _type: String,
    text: String,
}

pub async fn chat(prompt: String) -> Result<String, String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .map_err(|_| "ANTHROPIC_API_KEY saknas".to_string())?;
    let client = Client::new();

    let request_body = ChatRequest {
        model: "claude-3-5-haiku-20241022".to_string(),
        max_tokens: 1024,
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let chat_response: ChatResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    chat_response
        .content
        .get(0)
        .map(|c| c.text.clone())
        .ok_or_else(|| "Empty content in response".to_string())
}