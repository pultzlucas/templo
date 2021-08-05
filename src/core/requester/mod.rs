mod requester;

use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct AuthResponse {
    pub authenticated: bool,
    pub key: String,
    pub message: String,
}

pub struct RegisterResponse {
    pub registered: bool,
    pub key: String,
    pub message: String,
}

pub use requester::ProtternRequester;
pub use hyper::Method;
pub use hyper::http::HeaderValue;
