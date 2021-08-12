use super::{TempContent, TempPath, TempPathType};
use crate::core::utils::path::{pathbuf_to_string, valid_directory_path, remove_dir_prefix};
use fs_tree::FsTreeBuilder;
use std::{fs, io::Error};

pub fn mine_template_from(directory: &str) -> Result<Vec<TempPath>, Error> {
    valid_directory_path(directory)?;
    let fs_tree = FsTreeBuilder::new(directory).build();
    let vec_fs_tree: Vec<TempPath> = fs_tree
        .into_iter()
        .map(|path| TempPath::new(path.unwrap()))
        .collect();
    Ok(vec_fs_tree)
}

pub fn extract_files_from_paths(paths: Vec<TempPath>, directory: &str) -> Vec<TempContent> {
    paths
        .into_iter()
        .filter(|path| path.path_type == TempPathType::File)
        .map(|file| {
            let file_path: TempPath = remove_dir_prefix(file.clone(), directory).unwrap();
            println!("\n\n\n\n\n{:?}\n\n\n", fs::read_to_string(file.clone().buf).unwrap());
            TempContent {
                filename: pathbuf_to_string(file_path.buf),
                text: fs::read_to_string(file.buf).unwrap(),
            }
        })
        .collect()
}
