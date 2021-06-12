use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileContent {
    pub file: String,
    pub content: String,
}

impl FileContent {
    pub fn new(file: String, content: String) -> Self {
        Self { file, content }
    }
}
