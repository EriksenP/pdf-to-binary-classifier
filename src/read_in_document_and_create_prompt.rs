use std::{fs, path::PathBuf};

use crate::consts;
pub struct Prompt {
    pub task: String,
    pub language: String,
    pub description: String,
    pub context: Option<String>,
}

pub fn read_in_document_and_create_prompt(document_text: &PathBuf) -> Option<Prompt> {
    let contents = fs::read_to_string(document_text);
    if contents.is_err() {
        eprintln!(
            "Error reading document: {}",
            contents.unwrap_err().to_string()
        );
        return Option::None;
    }
    let contents = contents.unwrap();
    let prompt = Prompt {
        task: format!("{} {}", consts::LLM_PROMPT, contents),
        language: String::from("en"),
        description: String::from(
            "Summarize the text and generate binary classifiers for police mentions.",
        ),
        context: Option::None,
    };
    Some(prompt)
}
