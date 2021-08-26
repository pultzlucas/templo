mod manager;
mod permissions;

pub use manager::*;
pub use permissions::UserPermissions;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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