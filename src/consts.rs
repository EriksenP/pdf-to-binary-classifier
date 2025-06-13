pub const PDF_OCR_TOOL: &str = "ocrmypdf";
pub const PDF_OCR_ARG_1: &str = "--sidecar";
pub const PDF_OCR_ARG_LAST: &str = "trash.pdf";
pub const BASH_PDF_FOLDER_CLEANUP: &str =
    "/home/paul/susan/policeunion/policeunion/src/scripts/clean_filepath_whitespace";
pub const PDF_PATH: &str = "/home/paul/susan/policeunion/data";
pub const OCR_OUTPUT_PATH: &str = "/home/paul/susan/policeunion/data/output";

pub const LLM_TO_USE: &str = "deepseek-r1:latest";
pub const LLM_PROMPT: &str = "Read the text below, generate a summary of the text, and then generate 3 binary classifiers indicating whether the police were mentioned in the text, wether a police union was menioned in the text, and wether the mention of the police or union was positive negative or neutral with respect to the outcome of the event.  Please output the results separated by commas and nothing else.  TEXT STARTS BELOW THIS SENTENCE.";

pub const BINARY_CLASSIFIER_OUTPUT_HEADERS: &str = "file,police,union,positive";
pub const BINARY_CLASSIFIER_OUTPUT_DIRECTORY: &str = "/home/paul/susan/policeunion/data/output";
pub const BINARY_CLASSIFIER_OUTPUT_PATH: &str =
    "/home/paul/susan/policeunion/data/output/binary_classifiers.csv";
