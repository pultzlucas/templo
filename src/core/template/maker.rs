use crate::core::template::{miner, TemplateType};
use crate::core::user_account::UserAccountManager;
use chrono::prelude::Utc;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Template {
    pub metadata: TempMetadata,
    pub name: String,
    pub paths: Vec<PathBuf>,
    pub contents: Vec<miner::File>,
}

#[derive(Debug, PartialEq)]
pub struct TempData {
    pub name: String,
    pub paths: Vec<PathBuf>,
    pub contents: Vec<miner::File>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TempMetadata {
    pub owner: String,
    pub created_at: String,
    pub template_type: TemplateType,
}

pub fn create_template(temp_name: String, dir_path: String) -> Result<Template, Error> {
    let metadata = get_template_metadata()?;
    let data = get_template_data(temp_name, dir_path)?;
    Ok(Template {
        metadata,
        name: data.name,
        paths: data.paths,
        contents: data.contents,
    })
}

fn get_template_data(temp_name: String, dir_path: String) -> Result<TempData, Error> {
    let paths = miner::extract_paths_from(dir_path.as_str())?;
    let files = miner::extract_files_from_paths(paths.clone());
    let files_with_content = files
        .into_iter()
        .filter(|file| file.content != "")
        .collect();
    Ok(TempData {
        name: temp_name,
        paths,
        contents: files_with_content,
    })
}

fn get_template_metadata() -> Result<TempMetadata, Error> {
    let owner = UserAccountManager::get_user_account_data()?.username;
    let date_now = get_date_now_string();
    Ok(TempMetadata {
        owner,
        created_at: date_now,
        template_type: TemplateType::Local,
    })
}

fn get_date_now_string() -> String {
    Regex::new(r"\..+")
        .unwrap()
        .replace(&Utc::now().to_string(), "")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_should_return_a_valid_template_data() {
        let template = get_template_data(
            "temp-name".to_string(),
            "./src/core/tests/tree_files_only".to_string(),
        )
        .unwrap();

        assert_eq!(
            template,
            TempData {
                name: "temp-name".to_string(),
                paths: vec![
                    Path::new("./src/core/tests/tree_files_only").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
                    Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf()
                ],
                contents: vec![crate::core::template::miner::File {
                    filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                    content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}"
                        .to_string()
                }]
            }
        )
    }
}
