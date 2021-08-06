use crate::core::template::miner;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Template {
    pub name: String,
    pub paths: Vec<PathBuf>,
    pub contents: Vec<miner::File>,
}

pub fn create_template(temp_name: String, dir_path: String) -> Result<Template, Error> {
    let paths = miner::extract_paths_from(dir_path.as_str())?;
    let files = miner::extract_files_from_paths(paths.clone());
    let files_with_content = files
        .into_iter()
        .filter(|file| file.content != "")
        .collect();
    let template = Template {
        name: temp_name,
        paths,
        contents: files_with_content,
    };
    Ok(template)
}
