use std::{path::PathBuf, process::Command};
mod consts;
mod clean_input_folder;
mod ocr_and_return_path;

fn main() {
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
    clean_input_folder::clean_up_spaces_in_filenames(consts::PDF_PATH);

    // Step 2:
    // Get a list of every document in the folder
    // For error checking purposes we will perform OCR and LLM
    // Each document separately rather than doing the operations
    // in a batch.
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
        let document_text_path= ocr_and_return_path::perform_ocr(path, consts::OCR_OUTPUT_PATH);
        let document_text_path = match document_text_path {
            Some(path) => path,
            None => continue,
        };
        println!("Document text path: {}", document_text_path.to_str().unwrap())
    }
}


