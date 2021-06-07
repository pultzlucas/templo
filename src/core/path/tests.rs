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
    assert_eq!(format_paths_name("dir".to_string(), vec![
        DirPath {
            name: "dir\\file1.txt".to_string(),
            path_type: "file"
        },
        DirPath {
            name: "dir\\file2.txt".to_string(),
            path_type: "file"
        }
    ]), vec![
        "file|file1.txt".to_string(),
        "file|file2.txt".to_string()
    ])
}