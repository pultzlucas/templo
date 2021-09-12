use super::{TempContent, TempPath, TempPathType};
use crate::utils::path::{
    format_path_namespace, pathbuf_to_string, remove_dir_prefix, valid_directory_path,
};
use base64;
use fs_tree::FsTreeBuilder;
use std::{fs, io::Error};

pub fn mine_paths_from(directory_path: &str) -> Result<Vec<TempPath>, Error> {
    valid_directory_path(directory_path)?;
    let fs_tree = FsTreeBuilder::new(directory_path)
        .ignore_paths(&["./TemplateConfig"])
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
