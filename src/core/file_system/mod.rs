use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

mod file_system;
pub mod paths;
pub use file_system::ProtternFileSystem;

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

pub fn pathbuf_to_string(path: PathBuf) -> String {
    path.as_os_str().to_str().unwrap().to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileContent {
    pub filename: String,
    pub content: String,
}
