use crate::consts::{BASH_PDF_FOLDER_CLEANUP, PDF_PATH};
use std::process::Command;

pub fn clean_up_spaces_in_filenames(pdf_directory: &str) {
    println!("clean up step");
    // Step 1:
    // Clean up the input directory, removing spaces in file names.
    // I am doing this because working with spaces in file names is
    // ANNOYING and I saw a bunch of them in the first sample.
    println!(
        "Cleaning up spaces in file names in directory: {}",
        PDF_PATH
    );
    // verify script is executable first
    Command::new("chmod")
        .arg("+x")
        .arg(BASH_PDF_FOLDER_CLEANUP)
        .spawn()
        .expect("failed to chmod the file");

    // execute the clean up script
    Command::new("bash")
        .arg(BASH_PDF_FOLDER_CLEANUP)
        .current_dir(pdf_directory)
        .output()
        .expect("Failed to execute bash script");
}
