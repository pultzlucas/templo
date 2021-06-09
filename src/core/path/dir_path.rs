use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DirPath<'a> {
    pub name: String,
    pub path_type: &'a str,
}

impl<'a> DirPath<'a> {
    pub fn new(name: String, path_type: &'a str) -> Self {
        Self {name, path_type}
    }
}