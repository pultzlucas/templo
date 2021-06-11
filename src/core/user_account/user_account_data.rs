extern crate serde;
extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAccountData {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserAccountData {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}
