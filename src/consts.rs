pub const PDF_OCR_TOOL: &str = "ocrmypdf";
pub const PDF_OCR_ARG_1: &str = "--sidecar";
pub const PDF_OCR_ARG_LAST: &str = "trash.pdf";
pub const BASH_PDF_FOLDER_CLEANUP: &str =
    "/home/paul/susan/policeunion/policeunion/src/scripts/clean_filepath_whitespace";
pub const PDF_PATH: &str = "/home/paul/susan/policeunion/data";
pub const OCR_OUTPUT_PATH: &str = "/home/paul/susan/policeunion/data/output";

pub const LLM_TO_USE: &str = "deepseek-r1:latest";
pub const LLM_PROMPT: &str = "The following are a set of instructions:
1) Read the text below the 20 equal signs
2) Generate a summary of the text, which includes parties involved, nature of the dispute, and the resolution of the dispute.
3) Create a binary classifier for 3 pieces of information:
	a) Were the police involved?
	b) Was one or more police union involved?
	c) Was the resolution positive or negative for the police?
4) Structure your response in a json object that follows this structure (including tildes):
~~~~~
{
	summary: string
	police_involved: bool
	police_union_involved: bool
	outcome: positive, negative, neutral, none
},

====================";

pub const LLM_OUTPUT_HEADERS: &str = "";
pub const LLM_OUTPUT_PATH: &str =
    "/home/paul/susan/policeunion/data/output/llm_output.json";
