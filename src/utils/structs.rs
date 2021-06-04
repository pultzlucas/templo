extern crate serde;
extern crate serde_json;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub paths: String,
}