use crate::core::user_account::UserAccountManager;
use chrono::prelude::Utc;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TemplateType {
    Local,
    Remote
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub template_type: TemplateType, 
    pub owner: String,
    pub created_at: String,
    pub paths: String,
    pub content: String,
}

impl Template {
    pub fn new(name: String, paths: String, content: String) -> Self {
        let owner = UserAccountManager::get_user_account_data()
            .unwrap()
            .username;
        let created_at = Utc::now().to_string();
        let template_type = TemplateType::Local;

        Self {
            name,
            paths,
            content,
            owner,
            created_at,
            template_type
        }
    }
}
