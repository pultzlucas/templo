use std::fs::{self, File};
use std::io::{Error, Read};

pub fn read_bytes(filename: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();

    Ok(buffer)
}

pub fn file_content_is_text(filename: &str) -> bool {
    let bytes = read_bytes(filename).expect("Failed read file bytes.");
    String::from_utf8(bytes).is_ok()
}
