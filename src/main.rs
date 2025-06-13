use tokio;

use crate::consts::BINARY_CLASSIFIER_OUTPUT_PATH;
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
    write_out_data::create_file(consts::BINARY_CLASSIFIER_OUTPUT_DIRECTORY)
        .expect("Failed to create output file");
    write_out_data::append_to_file(
        consts::BINARY_CLASSIFIER_OUTPUT_DIRECTORY,
        consts::BINARY_CLASSIFIER_OUTPUT_HEADERS,
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

        // step 5: output to file
        let filename = document_text_path.file_name().unwrap().to_str().unwrap();
        let writeable = format!("{},{}\n", filename, response);
        write_out_data::append_to_file(BINARY_CLASSIFIER_OUTPUT_PATH, &writeable)
            .expect("Failed to append classfiered to output file");
    }
}
