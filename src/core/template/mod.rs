pub mod generator;
pub mod maker;
pub mod miner;

#[cfg(test)]
mod tests;

use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
//use std::ops::Deref;
use std::path::PathBuf;
use tabled::Tabled;

// STRUCTS

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Template {
    pub name: String,
    pub created_at: String,
    pub template_type: TemplateType,
    pub paths: Vec<TempPath>,
    pub contents: Vec<TempContent>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TempPath {
    pub path: PathBuf,
    pub path_type: TempPathType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TempContent {
    pub file_path: String,
    pub text: String,
}

#[derive(Tabled)]
pub struct TemplateDisplayInfo {
    pub name: String,
    pub created_at: String,
    pub template_type: TemplateType,
}

//ENUMS

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TemplateType {
    Local,
    Remote,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum TempPathType {
    File,
    Dir,
}

// IMPLEMENTATIONS

impl TempPath {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path: path.clone(),
            path_type: if path.is_file() {
                TempPathType::File
            } else {
                TempPathType::Dir
            },
        }
    }
}

impl TempContent {
    pub fn new(file_path: String, text: String) -> Self {
        Self { file_path, text }
    }
}

impl Display for TemplateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match *self {
            TemplateType::Local => write!(f, "Local"),
            TemplateType::Remote => write!(f, "Remote"),
        }
    }
}


impl Template {
    pub fn fmt(&self) -> TemplateDisplayInfo {
        TemplateDisplayInfo {
            name: self.name.clone(),
            created_at: self.created_at.clone(),
            template_type: self.template_type.clone(),
        }
    }
}
