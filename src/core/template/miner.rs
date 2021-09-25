use super::{TempContent, TempPath, TempPathType};
use crate::utils::{
    errors::std_error,
    path::{format_path_namespace, pathbuf_to_string, remove_dir_prefix, valid_directory_path},
};
use base64;
use fs_tree::FsTreeBuilder;
use serde_json::from_str;
use std::{fs, io::Error};

pub fn mine_paths_from(directory_path: &str) -> Result<Vec<TempPath>, Error> {
    valid_directory_path(directory_path)?;

    let mut paths_to_ignore = get_paths_to_ignore()?;
    paths_to_ignore.push("./TemplateConfig".to_string());
    let paths_to_ignore_formatted = format_paths_to_ignore(paths_to_ignore)?;

    let fs_tree = (FsTreeBuilder::new(directory_path))
        .ignore_paths(&paths_to_ignore_formatted[..])
        .build();

    let vec_fs_tree: Vec<TempPath> = fs_tree
        .into_iter()
        .map(|path| TempPath::new(path.expect("Not is possible find the folder.")))
        .collect();

    Ok(vec_fs_tree)
}

pub fn mine_files_from_paths(paths: Vec<TempPath>, directory: &str) -> Vec<TempContent> {
    paths
        .into_iter()
        .filter(|path| path.path_type == TempPathType::File)
        .map(|file| {
            let file_path: TempPath = remove_dir_prefix(file.clone(), directory).unwrap();
            let filename = pathbuf_to_string(format_path_namespace(file_path.path));
            let text = fs::read_to_string(file.path).expect("Error when read file content");
            TempContent::new(filename, base64::encode(text))
        })
        .collect()
}

fn get_paths_to_ignore() -> Result<Vec<String>, Error> {
    std_error(from_str(&fs::read_to_string(
        "./TemplateConfig/ignore.json",
    )?))
}

fn format_paths_to_ignore(paths: Vec<String>) -> Result<Vec<String>, Error> {
    Ok(paths
        .into_iter()
        .map(|path| {
            if !path.starts_with("./") {
                return format!("./{}", path);
            }
            path
        })
        .collect())
}
