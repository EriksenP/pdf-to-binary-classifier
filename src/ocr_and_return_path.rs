use crate::consts::{OCR_OUTPUT_PATH, PDF_OCR_ARG_1, PDF_OCR_ARG_LAST, PDF_OCR_TOOL, PDF_PATH};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn perform_ocr(pdf_file_path: PathBuf, ocr_output_path: &str) -> Option<PathBuf> {
    // make sure you have a place to output to.
    verify_output_path_exists_or_create_or_fail();
    // perform the OCR and return the path to the document
    let mut output_path = PathBuf::new();
    output_path.push(ocr_output_path);
    let filename = pdf_file_path.file_name();
    let filename = match filename {
        Some(filename) => filename.to_str(),
        None => {
            eprintln!("Error getting os filename");
            return Option::None;
        }
    };
    let filename = match filename {
        Some(filename) => filename,
        None => {
            eprintln!(
                "Error getting filename as string if there was an os filename above this is expected"
            );
            return Option::None;
        }
    };
    if filename == "" {
        eprintln!(
            "Filename is empty, this should not happen if the filename was successfully retrieved above"
        );
        return Option::None;
    }
    let mut trash_output = PathBuf::clone(&output_path);

    output_path.push(filename.replace(".pdf", ".txt"));
    trash_output.push(PDF_OCR_ARG_LAST);
    // for debugging purposes
    // println!("Output path: {:?}", output_path);
    // println!("Pdf file path: {:?}", pdf_file_path);
    // println!("Trash path: {:?}", trash_output);
    let command = format!(
        "{} {} {} {} {}",
        PDF_OCR_TOOL,
        PDF_OCR_ARG_1,
        output_path.to_str().unwrap(),
        pdf_file_path.to_str().unwrap(),
        trash_output.to_str().unwrap()
    );

    // for debugging purposes
    //println!("Command: {}", command);
    // This may look wrong, but I tried it by directly issuing the command
    // and I had some issues, so I made it a debuggable string.

    Command::new("bash")
        .current_dir(PDF_PATH)
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute OCR command");
    // println!("OCR output: {:?}", text);
    return Option::Some(output_path);
}

fn verify_output_path_exists_or_create_or_fail() {
    if !Path::new(&OCR_OUTPUT_PATH).exists() {
        std::fs::create_dir_all(&OCR_OUTPUT_PATH).expect("Failed to create output directory");
    }
}
