use crate::core::user_account::UserAccountManager;
use chrono::prelude::Utc;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub owner: String,
    pub paths: String,
    pub created_at: String,
    pub content: String,
}

impl Template {
    pub fn new(name: String, paths: String, content: String) -> Self {
        let owner = UserAccountManager::get_user_account_data()
            .unwrap()
            .username;
        let created_at = Utc::now().to_string();

        Self {
            name,
            paths,
            content,
            owner,
            created_at,
        }
    }
}
