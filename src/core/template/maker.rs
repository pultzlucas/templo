use super::{miner, File, TempMetadata, Template, TemplateType};
use crate::core::user_account::get_user_account_data;
use crate::core::utils::date::get_date_now_string;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct TempData {
    pub paths: Vec<PathBuf>,
    pub contents: Vec<File>,
}

pub fn make_template(temp_name: String, dir_path: String) -> Result<Template, Error> {
    let metadata = make_template_metadata(temp_name)?;
    let data = make_template_data(dir_path)?;
    Ok(Template {
        metadata,
        paths: data.paths,
        contents: data.contents,
    })
}

pub fn make_template_data(dir_path: String) -> Result<TempData, Error> {
    let paths = miner::extract_paths_from(dir_path.as_str())?;
    let files = miner::extract_files_from_paths(paths.clone())
        .into_iter()
        .filter(|file| file.content != "")
        .collect();
    Ok(TempData {
        paths,
        contents: files,
    })
}

fn make_template_metadata(temp_name: String) -> Result<TempMetadata, Error> {
    let owner = get_user_account_data()?.username;
    let date_now = get_date_now_string();
    Ok(TempMetadata {
        owner,
        name: temp_name,
        created_at: date_now,
        template_type: TemplateType::Local,
    })
}