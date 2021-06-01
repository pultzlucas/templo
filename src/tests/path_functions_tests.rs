use crate::core;

#[test]
fn test_get_template_paths() {
    let dir = "dir_tests\\simple-dir".to_string();
    assert_eq!(core::get_template_paths(dir), vec![
        "dir_tests\\simple-dir".to_string(),
        "dir_tests\\simple-dir\\file.txt".to_string()
    ])
}