use std::{
    fs,
    io::{self, Write},
};

pub fn create_file(filename: &str) -> io::Result<()> {
    let file = fs::File::create_new(filename);
    if file.is_err() {
        eprintln!("Error creating file: {}", filename);
        eprintln!("Error code: {}", file.err().unwrap());
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    Ok(())
}

pub fn append_to_file(filename: &str, data: &str) -> io::Result<()> {
    let mut file = fs::OpenOptions::new().append(true).open(filename)?;
    file.write_all(data.as_bytes())
}
