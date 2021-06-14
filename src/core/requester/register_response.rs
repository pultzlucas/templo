use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub registered: bool,
    pub key: String,
    pub message: String,
}