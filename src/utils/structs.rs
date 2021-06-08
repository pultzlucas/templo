extern crate serde;
extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub paths: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAccountData {
    pub username: String,
    pub email: String,
    pub password: String,
}