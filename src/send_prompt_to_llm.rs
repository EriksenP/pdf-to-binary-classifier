use crate::{consts};
use reqwest::Client;
use serde_json::json;

pub async fn send_prompt_to_llm(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Sending prompt to LLM step");

    let body = json!({
        "model": consts::LLM_TO_USE,
        "prompt": prompt,
        "think": false,
        "stream": false
    });

    let client = Client::new();
    let res = client
        .post("http://localhost:11434/api/generate")
        .body(body.to_string())
        .send()
        .await?;

    let text = res.text().await?;
    // println!("Response: {}", text);
    Ok(text)
}
