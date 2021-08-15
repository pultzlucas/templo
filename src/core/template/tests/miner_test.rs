use super::*;
use crate::core::template::{TempPath, TempPathType};
use miner::*;
use std::path::Path;

const TREE_FILES_ONLY: &'static str = "./src/core/tests/tree_files_only";

#[test]
fn it_should_return_the_tree_files_only_flatted() {
    let flat = mine_paths_from(TREE_FILES_ONLY).unwrap();
    assert_eq!(
        flat,
        vec![
            TempPath {
                buf: Path::new("./src/core/tests/tree_files_only\\.file4").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("./src/core/tests/tree_files_only\\file-2").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("./src/core/tests/tree_files_only\\file1").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("./src/core/tests/tree_files_only\\file_3").to_path_buf(),
                path_type: TempPathType::File
            },
            TempPath {
                buf: Path::new("./src/core/tests/tree_files_only\\file_text.txt").to_path_buf(),
                path_type: TempPathType::File
            },
        ]
    );
}

#[test]
fn it_should_return_all_tree_files_only_files() {
    let paths = mine_paths_from(TREE_FILES_ONLY).unwrap();
    let contents: Vec<TempContent> = mine_files_from_paths(paths, TREE_FILES_ONLY)
        .into_iter()
        .filter(|file_content| file_content.text != "")
        .collect();

    assert_eq!(
        contents,
        vec![TempContent {
            file_path: "file_text.txt".to_string(),
            text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
        }]
    )
}
