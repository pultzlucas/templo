use super::*;
use miner::*;
use crate::core::utils::path::*;
use std::path::Path;
const TREE_FILES_ONLY: &'static str = "./src/core/tests/tree_files_only";

#[test]
fn it_should_convert_pathbuf_to_string() {
    assert_eq!(
        pathbuf_to_string(Path::new(TREE_FILES_ONLY).to_path_buf()),
        TREE_FILES_ONLY
    )
}

#[test]
fn it_should_return_the_tree_files_only_flatted() {
    let flat = extract_paths_from(TREE_FILES_ONLY).unwrap();
    assert_eq!(
        flat,
        vec![
            Path::new("./src/core/tests/tree_files_only"),
            Path::new("./src/core/tests/tree_files_only/.file4"),
            Path::new("./src/core/tests/tree_files_only/file-2"),
            Path::new("./src/core/tests/tree_files_only/file1"),
            Path::new("./src/core/tests/tree_files_only/file_3"),
            Path::new("./src/core/tests/tree_files_only/file_text.txt")
        ]
    );
}

#[test]
fn it_should_return_all_tree_files_only_files() {
    let paths = extract_paths_from(TREE_FILES_ONLY).unwrap();
    let contents: Vec<File> = extract_files_from_paths(paths)
        .into_iter()
        .filter(|file_content| file_content.content != "")
        .collect();

    assert_eq!(
        contents,
        vec![File {
            filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
            content: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
        }]
    )
}