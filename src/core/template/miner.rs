use crate::core::utils::errors::std_error;
use crate::core::utils::path::{format_path_namespace, pathbuf_to_string, valid_directory_path};
use super::TempContent;
use fs_tree::FsTreeBuilder;
use std::{fs, io::Error, path::PathBuf};

pub fn extract_paths_from(directory: &str) -> Result<Vec<PathBuf>, Error> {
    valid_directory_path(directory)?;
    let fs_tree = FsTreeBuilder::new(directory).build();
    let vec_fs_tree: Result<Vec<PathBuf>, Error> = fs_tree
        .into_iter()
        .map(|path| format_path_namespace(path.unwrap()))
        .map(|path| remove_dir_prefix(path, directory))
        .collect();
    vec_fs_tree
}

pub fn extract_files_from_paths(paths: Vec<PathBuf>) -> Vec<TempContent> {
    paths
        .into_iter()
        .filter(|path| path.is_file())
        .map(|file| TempContent {
            filename: pathbuf_to_string(file.clone()),
            text: fs::read_to_string(file).unwrap(),
        })
        .collect()
}

fn remove_dir_prefix(path: PathBuf, directory: &str) -> Result<PathBuf, Error> {
    let prefix = format!("{}/", directory);
    Ok(std_error(path.strip_prefix(prefix))?.to_path_buf())
}
