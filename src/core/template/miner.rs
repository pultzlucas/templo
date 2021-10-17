use super::TempPath;
use crate::core::{
    utils::errors::std_error,
    utils::path::{pathbuf_to_string, valid_directory_path},
};
use fs_tree::FsTreeBuilder;
use serde_json::from_str;
use std::{fs, io::Error, path::Path};

pub fn mine_paths_from(directory_path: &str) -> Result<Vec<TempPath>, Error> {
    valid_directory_path(directory_path)?;

    let paths_to_ignore = get_paths_to_ignore(directory_path)?;

    let fs_tree = (FsTreeBuilder::new(directory_path))
        .ignore_paths(&paths_to_ignore[..])
        .build();

    fs_tree
        .into_iter()
        .map(|path| TempPath::create(path.expect("Not is possible find the path.")))
        .collect()
}

fn get_paths_to_ignore(directory_path: &str) -> Result<Vec<String>, Error> {
    let template_config = Path::new(directory_path).join("TemplateConfig");

    if !template_config.exists() {
        return Ok(vec!["".to_string()]);
    }

    let ignore_filename = template_config.join("ignore.json");

    let mut paths_to_ignore = if ignore_filename.exists() {
        std_error(from_str(&fs::read_to_string(ignore_filename)?))?
    } else {
        vec![]
    };
    paths_to_ignore.push("./TemplateConfig/".to_string());

    let real_paths_to_ignore: Vec<String> = paths_to_ignore
        .into_iter()
        .map(|path| Path::new(directory_path).join(path))
        .map(pathbuf_to_string)
        .collect();

    Ok(real_paths_to_ignore)
}
