extern crate serde;
extern crate serde_json;
use crate::core::user_account::UserAccountManager;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub owner: String,
    pub paths: String,
    pub content: String,
}

impl Template {
    pub fn new(name: String, paths: String, content: String) -> Self {
        let owner = UserAccountManager::get_user_account_data()
            .unwrap()
            .username;
        Self {
            name,
            paths,
            content,
            owner,
        }
    }
}
