mod manager;
pub mod miner;
pub mod maker;
pub mod serde;

pub use manager::*;

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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Tabled)]
pub struct TempMetadata {
    pub owner: String,
    pub created_at: String,
    pub template_type: TemplateType,
}