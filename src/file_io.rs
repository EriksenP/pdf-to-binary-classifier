use std::{
    collections::HashSet, fs, io::{self, BufRead, Write}, path::Path
};

use serde_json::Value;


struct Checkpoint {
    pub completed: HashSet<String>,
    // TODO: check what we already OCRed.
    // this is pretty fast so I won't waste time
    // on it now.
}

impl Checkpoint {
    pub fn new() -> Self {
        Self {
            completed: HashSet::new(),
        }
    }

    pub fn create (completed: HashSet<String>) -> Self {
        Self { completed }
    }
}

// the following two functions read in the lines that we have completed
// and creates a set of completed files.
pub fn parse_input_line_from_outputter(line: &str) -> String {
    let obj:Result<Value, _> = serde_json::from_str(line);
    match obj {
        Ok(json) => json["filename"].as_str().unwrap().to_string(),
        Err(_) => "".to_string(), // do not care about returning a black string here.
    }
}
fn check_previous_run_in_output_file(filename: &str) -> Result<HashSet<String>, io::Error> {
    let mut lines = HashSet::new();
    let file = fs::File::open(filename)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.insert(parse_input_line_from_outputter(line.as_str()));
                // this looks ugly...
                ()
            },
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Failed to read line"))
        }
    }

    Ok(lines)
}

pub fn create_file_or_get_current_position(filename: &str) -> Result<Checkpoint, io::Error>{
    if fs::metadata(Path::new(filename)).is_ok() {
        // file exists
        // read the file and figure out how many entries and which have been processed
        let values_from_file = check_previous_run_in_output_file(filename);
        match values_from_file {
            Ok(values) => Ok(Checkpoint::create(values)),
            Err(e) => Err(e),
        }
    } else {
        let res = create_file(filename);
        match res {
            Ok(_) => Ok(Checkpoint::new()),
            Err(e) => {
                eprintln!("Error creating file: {}", filename);
                Err(e)
            }
        }
    }
}

fn create_file(filename: &str) -> io::Result<()> {
    let file = fs::File::create_new(filename);
    if file.is_err() {
        eprintln!("Error creating file: {}", filename);
        eprintln!("Error code: {}", file.err().unwrap());
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    Ok(())
}

pub fn append_to_file(filename: &str, data: &str) -> io::Result<()> {
    println!("Appending data to file: {}", data);
    let mut file = fs::OpenOptions::new().append(true).open(filename)?;
    let w = file.write_all(data.as_bytes());
    if w.is_ok() {
        file.flush().expect("Failed to flush the buffer");
    }
    w
}
