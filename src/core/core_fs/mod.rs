use std::fs::{self, File};
use std::io::{Error, Read};
use std::path::Path;

pub fn read_bytes<P: AsRef<Path> + Copy>(filename: P) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filename)?;
    let metadata = fs::metadata(filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).unwrap();

    Ok(buffer)
}
