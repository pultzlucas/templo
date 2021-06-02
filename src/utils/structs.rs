extern crate serde;
extern crate serde_json;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HEAD {
    pub name: String,
    pub paths: String,
}