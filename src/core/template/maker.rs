use super::config::get_config_args;
use super::{miner, TempContent, TempPath, Template};
use crate::utils::date::get_date_now_string;
use crate::utils::path::{format_path_namespace, pathbuf_to_string, remove_dir_prefix};
use std::io::Error;

#[derive(Debug, PartialEq)]
pub struct TempData {
    pub paths: Vec<TempPath>,
    pub contents: Vec<TempContent>,
}

pub fn make_template(
    temp_name: String,
    description: Option<String>,
    ref_path: &str,
) -> Result<Template, Error> {
    let (name, created_at) = make_template_metadata(temp_name)?;
    let data = make_template_data(ref_path)?;
    let args = get_config_args(ref_path)?;
    Ok(Template {
        name,
        description,
        created_at,
        paths: data.paths,
        contents: data.contents,
        args,
    })
}

pub fn make_template_data(dir_path: &str) -> Result<TempData, Error> {
    let raw_paths = miner::mine_paths_from(dir_path)?;

    let files = miner::mine_files_from_paths(raw_paths.clone(), dir_path)
        .into_iter()
        .filter(|file| file.text != "")
        .collect();

    let formatted_paths: Vec<TempPath> = raw_paths
        .into_iter()
        .map(|path| TempPath {
            path: format_path_namespace(path.path),
            path_type: path.path_type,
        })
        .map(|path| remove_dir_prefix(path, dir_path).unwrap())
        .filter(|path| pathbuf_to_string(path.path.clone()) != "")
        .collect();
    Ok(TempData {
        paths: formatted_paths,
        contents: files,
    })
}

type TempMetadata = (String, String);
fn make_template_metadata(temp_name: String) -> Result<TempMetadata, Error> {
    let created_at = get_date_now_string();
    Ok((temp_name, created_at))
}
