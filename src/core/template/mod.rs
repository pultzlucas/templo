mod manager;
pub mod miner;
pub mod maker;
pub mod serde;

#[cfg(test)]
mod tests;

pub use manager::*;

use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use tabled::Tabled;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct File {
    pub filename: String,
    pub content: String,
}

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
    pub paths: Vec<PathBuf>,
    pub contents: Vec<File>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Tabled)]
pub struct TempMetadata {
    pub name: String,
    pub owner: String,
    pub created_at: String,
    pub template_type: TemplateType,
}