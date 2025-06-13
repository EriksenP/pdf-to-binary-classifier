use crate::{consts, read_in_document_and_create_prompt::Prompt};
use reqwest::Client;
use serde_json::json;

pub async fn send_prompt_to_llm(prompt: &Prompt) -> Result<String, Box<dyn std::error::Error>> {
    println!("Sending prompt to LLM step");
    let prompt_str = format!(
        "Task: {}\nLanguage: {}\nDescription: {}\nContext: {}",
        prompt.task,
        prompt.language,
        prompt.description,
        prompt.context.clone().unwrap_or_default()
    );

    let body = json!({
        "model": consts::LLM_TO_USE,
        "prompt": prompt_str,
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
