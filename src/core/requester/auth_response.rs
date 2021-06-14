use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub authenticated: bool,
    pub key: String,
    pub message: String,
}