use super::maker::*;
use std::path::Path;

#[test]
fn it_should_return_a_valid_template_data() {
    let temp_data = make_template_data("./src/core/tests/tree_files_only".to_string()).unwrap();

    assert_eq!(
        temp_data,
        TempData {
            paths: vec![
                Path::new("./src/core/tests/tree_files_only").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/.file4").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file-2").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file1").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file_3").to_path_buf(),
                Path::new("./src/core/tests/tree_files_only/file_text.txt").to_path_buf()
            ],
            contents: vec![crate::core::template::TempContent {
                filename: "./src/core/tests/tree_files_only/file_text.txt".to_string(),
                text: "Lorem ipsum dolor\r\n\r\n123123123123\r\n\r\n{(-@#$%¨¨&*)}".to_string()
            }]
        }
    )
}
