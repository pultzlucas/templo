use serde_derive::{Deserialize, Serialize};

mod file_system;
pub mod paths;
pub use file_system::ProtternFileSystem;
use std::{fs, io::Error, path::Path};

pub fn write_base64_file<P: AsRef<Path>>(path: P, content: String) -> Result<(), Error> {
    let content_as_base64 = base64::encode(content);
    fs::write(path, content_as_base64)
}

pub fn read_base64_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let content = {
        let content_as_base64 = fs::read_to_string(path)?;
        base64::decode(content_as_base64).expect("Base64 decode error")
    };
    Ok(String::from_utf8(content).unwrap())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DirPath<'a> {
    pub name: String,
    pub path_type: &'a str,
}

impl<'a> DirPath<'a> {
    pub fn new(name: String, path_type: &'a str) -> Self {
        Self { name, path_type }
    }

    pub fn deserialize(path: String) -> (String, String) {
        let path_split: Vec<&str> = path.split("|").collect();
        (
            path_split[1].to_string().clone(),
            path_split[0].to_string().clone(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileContent {
    pub filename: String,
    pub content: String,
}
