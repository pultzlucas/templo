pub mod config;
pub mod generator;
pub mod getter;
pub mod maker;
pub mod miner;

use super::{core_fs, http};

#[cfg(test)]
mod tests;

use config::ConfigArg;
use serde_derive::{Deserialize, Serialize};
use std::{io::Error, path::PathBuf};
use tabled::Tabled;

// STRUCTS

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Template {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub paths: Vec<TempPath>,
    pub args: Option<Vec<ConfigArg>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TempPath {
    pub path: PathBuf,
    pub is_file: bool,
    pub content: Option<TempPathContent>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TempPathContent {
    pub bytes: String,
    pub is_text: bool,
}

#[derive(Tabled)]
pub struct TemplateDisplayInfo {
    pub name: String,
    pub author: String,
    pub created_at: String,
}

// IMPLEMENTATIONS

impl TempPath {
    pub fn create(path: PathBuf) -> Result<Self, Error> {
        let content = if path.is_file() {
            let bytes = core_fs::read_bytes(&path)?;
            Some(TempPathContent::new(bytes))
        } else {
            None
        };

        Ok(Self {
            path: path.clone(),
            is_file: path.is_file(),
            content,
        })
    }
}

impl TempPathContent {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: base64::encode(&bytes),
            is_text: String::from_utf8(bytes).is_ok(),
        }
    }
}

impl Template {
    pub fn fmt(&self) -> TemplateDisplayInfo {
        TemplateDisplayInfo {
            name: self.name.clone(),
            author: if let Some(author) = &self.author {
                author.to_owned()
            } else {
                "unknown".to_string()
            },
            created_at: self.created_at.clone(),
        }
    }

    pub fn files(&self) -> Vec<TempPath> {
        self.paths
            .clone()
            .into_iter()
            .filter(|path| path.is_file)
            .collect()
    }
}
