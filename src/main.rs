use std::process;

use serde_json::{json, Value};
use regex::Regex;
use tokio;

use crate::consts::LLM_OUTPUT_PATH;
mod clean_input_folder;
mod consts;
mod ocr_and_return_path;
mod read_in_document_and_create_prompt;
mod send_prompt_to_llm;
mod write_out_data;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // stages of this project:
    // 1) OCR PDF
    //  a) Do this via a library
    //  b) Do this via an LLM like Vision
    // 2) Send PDF to LLM to perform a summary
    // 3) Send PDF to LLM to perform binary classification
    // 4) Amalgamate the results in a CSV
    // 5) Write out the results

    // Solutions
    // 1) Used ocrmypdf 13.4.0+dfsg to perform the OCR

    // Step 1:
    // Clean up the input directory, removing spaces in file names.
    // I am doing this because working with spaces in file names is
    // ANNOYING and I saw a bunch of them in the first sample.
    // also create the output file if it doesn't exist.
    clean_input_folder::clean_up_spaces_in_filenames(consts::PDF_PATH);
    write_out_data::create_file(consts::LLM_OUTPUT_PATH)
        .expect("Failed to create output file");
    write_out_data::append_to_file(
        consts::LLM_OUTPUT_PATH,
        consts::LLM_OUTPUT_HEADERS,
    )
    .expect("Failed to wrtie headers to output file");
    // Step 2:
    // Get a list of every document in the folder
    // For error checking purposes we will perform OCR and LLM
    // Each document separately rather than doing the operations
    // in a batch.  This is per Susan so that she can check the outputs.

    // read in all of the documents in the directory
    // TODO: don't assume they are all PDFs.
    let documents = std::fs::read_dir(consts::PDF_PATH).unwrap();
    for document in documents {
        // need to think about this error case more.
        if document.is_err() {
            eprintln!(
                "Error reading document: {}",
                document.unwrap_err().to_string()
            );

            continue;
        }
        let document = document.unwrap();
        let path = document.path();
        let document_text_path = ocr_and_return_path::perform_ocr(path, consts::OCR_OUTPUT_PATH);
        let document_text_path = match document_text_path {
            Some(path) => path,
            None => continue,
        };
        println!(
            "Document text path: {}",
            document_text_path.to_str().unwrap()
        );

        // step 3: Read in the OCRed document and create a prompt for the LLM
        let prompt = read_in_document_and_create_prompt::read_in_document_and_create_prompt(
            &document_text_path,
        );
        if prompt.is_none() {
            eprintln!(
                "Error creating prompt for document: {}",
                document_text_path.to_str().unwrap()
            );
            continue;
        }
        let prompt = prompt.unwrap();

        // step 4: Send the prompt to the LLM and get a response
        let response = send_prompt_to_llm::send_prompt_to_llm(&prompt).await;
        if response.is_err() {
            eprintln!("Error sending prompt to LLM: {}", response.err().unwrap());
            continue;
        }
        let response = response.unwrap();

        // Getting deepseek to not include its thoughts seems difficult.
        // I am going to process the text and remove everything but what I want.
        
        // get the text of the response
        let text = response.as_str();
        // get the values as a json object
        let full_llm_response: Value = serde_json::from_str(text).unwrap();
        // get the response value from the text
        let text = full_llm_response.get("response").unwrap().to_string();
        // look for the jeson in the response text
        let json_in_response_pattern = Regex::new(r"\{[^{}]*\}").unwrap();
        // If we find the json then parse it otherwise fail
        if let Some(proper) = json_in_response_pattern.find(&text) {
            let possible = proper.as_str();
            // this is annoying but the parser is REALLY specific.  Have to clean up
            // all the formatting and escape chars before parsing it.
            let temp = clean_str(possible);
            let possible = &temp.as_str();

            println!("cleaned json: {}", possible);
            let json_response:Result<Value, _> = serde_json::from_str(possible);
            if json_response.is_ok() {
                let mut json_response = json_response.unwrap();
                println!("Created json response: {}", json_response);
                // create the filename
                let filename = document_text_path.file_name().unwrap().to_str().unwrap();
                // jam it into the json
                json_response["filename"] = json!(filename);
                let text = json_response.to_string();
                println!("final text: {}", text);
                // step 5: output to file
                let writeable = format!("{}\n", text);
                write_out_data::append_to_file(LLM_OUTPUT_PATH, &writeable)
                    .expect("Failed to append classfiered to output file");
            } else {
                println!("Json response was not okay!");
                println!("error: {}", json_response.err().unwrap());
            }
        }else {
            println!("Failed to find json in the response... serious issue");
            process::exit(-1); 
        }
    }
}

fn clean_str(input: &str) -> String {
    input.replace("\\n", "")
        .replace("\\t", "")
         .replace("\\r", "")
        .replace("\n", "")
         .replace("\t", "")
         .replace("\r", "")
         .replace("\\","")
}