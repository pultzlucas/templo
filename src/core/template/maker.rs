use super::{miner, TempContent, TempMetadata, TempPath, Template, TemplateType};
use crate::core::user_account::get_user_account_data;
use crate::core::utils::date::get_date_now_string;
use crate::core::utils::path::{pathbuf_to_string, remove_dir_prefix};
use std::io::Error;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct TempData {
    pub paths: Vec<TempPath>,
    pub contents: Vec<TempContent>,
}

pub fn make_template(temp_name: String, dir_path: &str) -> Result<Template, Error> {
    let metadata = make_template_metadata(temp_name)?;
    let data = make_template_data(dir_path)?;
    Ok(Template {
        metadata,
        paths: data.paths,
        contents: data.contents,
    })
}

pub fn make_template_data(dir_path: &str) -> Result<TempData, Error> {
    let raw_paths = miner::mine_template_from(dir_path)?;     

    let files = miner::extract_files_from_paths(raw_paths.clone(), dir_path)
        .into_iter()
        .filter(|file| file.text != "")
        .collect();

    let formatted_paths: Vec<TempPath> = raw_paths.into_iter()
        .map(|path| format_path_namespace(path))
        .map(|path| remove_dir_prefix(path, dir_path).unwrap())
        .filter(|path| pathbuf_to_string(path.buf.clone()) != "")
        .collect();
    
    Ok(TempData {
        paths: formatted_paths,
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

fn format_path_namespace(path: TempPath) -> TempPath {
    let path_formatted = Path::new(&pathbuf_to_string(path.buf).replace(r"\", "/")).to_path_buf();
    TempPath::new(path_formatted)
}


