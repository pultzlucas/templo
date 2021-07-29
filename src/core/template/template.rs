use crate::core::user_account::UserAccountManager;
use chrono::prelude::Utc;
use std::fmt::{Display, Result, Formatter};
use tabled::Tabled;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TemplateType {
    Local,
    Remote
}

impl Display for TemplateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TemplateType::Local => write!(f, "Local"),
            TemplateType::Remote => write!(f, "Remote"),
        }
    }
}

#[derive(Tabled)]
pub struct TemplateDisplayInfo {
    template_name: String,
    owner: String,
    template_type: TemplateType,
    created_at: String
}

#[derive(Tabled)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub owner: String,
    pub template_type: TemplateType, 
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

    pub fn has_content(&self) -> bool {
        self.content != ""
    }
}
