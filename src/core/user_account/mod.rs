mod user_account_manager;
mod user_permissions;

pub use user_account_manager::{UserAccountKey, UserAccountManager};
pub use user_permissions::UserPermissions;

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