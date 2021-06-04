use super::*;

#[test]
#[ignore]
fn test_get_template_paths() {
    let dir = "./dir_tests/simple-dir".to_string();
    assert_eq!(get_template_paths(dir).unwrap(), vec![
        "file.txt".to_string()
    ])
}

#[test]
#[ignore]
fn test_clear_files_name() {
    assert_eq!(clear_files_name("dir".to_string(), vec![
        "dir\\file1.txt".to_string(),
        "dir\\file2.txt".to_string()
    ]), vec![
        "file1.txt".to_string(),
        "file2.txt".to_string()
    ])
}