#[warn(unused_imports)]
use crate::core;

#[test]
fn test_get_template_paths() {
    let dir = "dir_tests\\simple-dir".to_string();
    assert_eq!(core::get_template_paths(dir).unwrap(), vec![
        "dir_tests\\simple-dir".to_string(),
        "dir_tests\\simple-dir\\file.txt".to_string()
    ])
}

#[test]
fn test_clear_files_name() {
    assert_eq!(core::clear_files_name("./dir/".to_string(), vec![
        "./dir/file1.txt".to_string(),
        "./dir/file2.txt".to_string()
    ]), vec![
        "file1.txt".to_string(),
        "file2.txt".to_string()
    ])
}