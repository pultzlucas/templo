use super::*;
use std::fs;

#[test]
fn it_should_write_and_read_a_b64_file() {
    write_base64_file("./test-base64.txt", "text for base64".to_string()).unwrap();
    let file_b64 = fs::read_to_string("./test-base64.txt").unwrap();
    assert_eq!(file_b64, "dGV4dCBmb3IgYmFzZTY0");

    let file_string = read_base64_file("./test-base64.txt").unwrap();
    assert_eq!(file_string, "text for base64".to_string());

    //delete test file
    fs::remove_file("./test-base64.txt").unwrap();
}

