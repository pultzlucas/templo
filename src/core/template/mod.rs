pub mod generator;
pub mod maker;
pub mod miner;
pub mod engine;
pub mod config;

#[cfg(test)]
mod tests;

use config::ConfigArg;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use tabled::Tabled;

// STRUCTS

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Template {
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub paths: Vec<TempPath>,
    pub contents: Vec<TempContent>,
    pub args: Option<Vec<ConfigArg>>
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
}

//ENUMS

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

impl Template {
    pub fn fmt(&self) -> TemplateDisplayInfo {
        TemplateDisplayInfo {
            name: self.name.clone(),
            created_at: self.created_at.clone(),
        }
    }
}
