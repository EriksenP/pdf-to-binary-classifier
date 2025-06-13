pub const PDF_OCR_TOOL: &str = "ocrmypdf";
pub const PDF_OCR_ARG_1: &str = "--sidecar";
pub const PDF_OCR_ARG_LAST: &str = "trash.pdf";
pub const BASH_PDF_FOLDER_CLEANUP: &str =
    "/home/paul/susan/policeunion/policeunion/src/scripts/clean_filepath_whitespace";
pub const PDF_PATH: &str = "/home/paul/susan/policeunion/data";
pub const OCR_OUTPUT_PATH: &str = "/home/paul/susan/policeunion/data/output";

pub const LLM_PROMPT: &str = "Read the text below, generate a summary of the text, and then generate 2 binary classifiers indicating whether the police or a police union were menioned in the text, and wether the mention of the police was positive negative or neutral with respect to the outcome of the event.  Please output the results separated by commas and nothing else.  TEXT STARTS BELOW THIS SENTENCE.";