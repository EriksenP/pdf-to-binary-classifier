use std::{
    fs,
    io::{self, Write}, path::Path,
};

pub fn create_file_or_get_current_position(filename: &str) -> Result<i32, io::Error>{
    if fs::metadata(Path::new(filename)).is_ok() {
        // file exists
        // read the file and figure out how many entries and which have been processed
        // TODO: make this function.
        // The idea is that you will loop through the lines and see which ones
        // have already been processed and then tell the system to skip those.
        // This will obviously require more than the i32 provided here.
        Ok(0)
    } else {
        let res = create_file(filename);
        match res {
            Ok(_) => Ok(0),
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
