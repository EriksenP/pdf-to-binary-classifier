use std::{fs, path::PathBuf};

use crate::consts;
pub fn read_in_document_and_create_prompt(document_text: &PathBuf) -> Option<String> {
    println!("Read in and create prompt step");
    let contents = fs::read_to_string(document_text);
    if contents.is_err() {
        eprintln!(
            "Error reading document: {}",
            contents.unwrap_err().to_string()
        );
        return Option::None;
    }
    let contents = contents.unwrap();
    // println!("{} {}", consts::LLM_PROMPT, contents);
    let words = format!("{} {}", consts::LLM_PROMPT, contents);
    Some(words)
}
