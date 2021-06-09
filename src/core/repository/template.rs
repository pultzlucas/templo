extern crate serde;
extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub paths: String,
    pub content: Vec<String>
}

impl Template {
    pub fn new(name: String, paths: String, content: Vec<String>) -> Self {
        Self { name, paths, content }
    }
}