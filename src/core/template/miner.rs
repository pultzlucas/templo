use crate::core::utils::path::{format_pathbuf, pathbuf_to_string, valid_directory_path};
use fs_tree::FsTreeBuilder;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Error, path::PathBuf};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct File {
    pub filename: String,
    pub content: String,
}

pub fn extract_paths_from(directory: &str) -> Result<Vec<PathBuf>, Error> {
    valid_directory_path(directory)?;
    let fs_tree = FsTreeBuilder::new(directory).build();
    let vec_fs_tree: Vec<PathBuf> = fs_tree
        .into_iter()
        .map(|path| format_pathbuf(path.unwrap()))
        .collect();
    Ok(vec_fs_tree)
}

pub fn extract_files_from_paths(paths: Vec<PathBuf>) -> Vec<File> {
    paths
        .into_iter()
        .filter(|path| path.is_file())
        .map(|file| File {
            filename: pathbuf_to_string(file.clone()),
            content: fs::read_to_string(file).unwrap(),
        })
        .collect()
}
