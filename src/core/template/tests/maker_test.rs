use super::maker::*;
use crate::core::template::{TempPath, TempPathType};
use std::path::Path;

const TREE_FILES_ONLY: &'static str = "./src/core/tests/tree_files_only";

#[test]
fn it_should_return_a_valid_template_data() {
    let temp_data = make_template_data(TREE_FILES_ONLY).unwrap();

    assert_eq!(
        temp_data,
        TempData {
            paths: vec![
                TempPath {
                    buf: Path::new(".file4").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file-2").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file1").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file_3").to_path_buf(),
                    path_type: TempPathType::File
                },
                TempPath {
                    buf: Path::new("file_text.txt").to_path_buf(),
                    path_type: TempPathType::File
                },
            ],
            contents: vec![crate::core::template::TempContent {
                filename: "file_text.txt".to_string(),
                text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
            }]
        }
    )
}
