mod bundler;
mod manager;
pub mod miner;
pub mod maker;
pub mod serde;

pub use manager::*;
pub use bundler::*;

use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use tabled::Tabled;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TemplateType {
    Local,
    Remote,
}

impl Display for TemplateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TemplateType::Local => write!(f, "Local"),
            TemplateType::Remote => write!(f, "Remote"),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Template {
    pub metadata: TempMetadata,
    pub name: String,
    pub paths: Vec<PathBuf>,
    pub contents: Vec<miner::File>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct TempMetadata {
    pub owner: String,
    pub created_at: String,
    pub template_type: TemplateType,
}


#[derive(Tabled)]
pub struct TemplateDisplayInfo {
    template_name: String,
    owner: String,
    template_type: TemplateType,
    created_at: String,
}

/* #[derive(Tabled, Debug, Serialize, Deserialize, Clone)]
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
        // get template owner
        let owner = UserAccountManager::get_user_account_data()
            .unwrap()
            .username;

        // get created at date
        let regex = Regex::new(r"\..+").unwrap();
        let created_at = regex
            .replace(&Utc::now().to_string(), "")
            .trim()
            .to_string();
        let template_type = TemplateType::Local;

        Self {
            name,
            paths,
            content,
            owner,
            created_at,
            template_type,
        }
    }

    pub fn has_content(&self) -> bool {
        self.content != ""
    }
}
 */