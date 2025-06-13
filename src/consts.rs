pub const PDF_OCR_TOOL: &str = "ocrmypdf";
pub const PDF_OCR_ARG_1: &str = "--sidecar";
pub const PDF_OCR_ARG_LAST: &str = "trash.pdf";
pub const PDF_OCR_JOB_ARGS: &str = "--jobs 10";
pub const BASH_PDF_FOLDER_CLEANUP: &str =
    "/home/paul/susan/policeunion/policeunion/src/scripts/clean_filepath_whitespace";
pub const PDF_PATH: &str = "/home/paul/susan/policeunion/data";
pub const OCR_OUTPUT_PATH: &str = "/home/paul/susan/policeunion/data/output";

pub const LLM_TO_USE: &str = "gemma3:latest";
pub const LLM_PROMPT: &str = "/no_think /nothink
The following are a set of instructions.  Follow all instuctions to the letter and do not deviate for any reason:
BEGIN INSTRUCTIONS
1) Read the text below the instructions
2) Generate a summary of the text with a length of 300 words or less, which includes parties involved, nature of the dispute, and the resolution of the dispute.
3) Gather 3 pieces of information:
	a) Were the police involved?
	b) Was one or more police union involved?
	c) Was the resolution positive or negative for the police or the police union?
4) No matter what, structure your response in a valid json object that has the following structure:
{
	summary: string
	police_involved: bool
	police_union_involved: bool
	outcome: positive, negative, neutral, none
}
5) Include no other text or information than the json object in step 4.
6) Never include any other text than the specified object in your response.
END INSTRUCTIONS
";

pub const LLM_OUTPUT_HEADERS: &str = "";
pub const LLM_OUTPUT_PATH: &str =
    "/home/paul/susan/policeunion/data/output/llm_output.json";
