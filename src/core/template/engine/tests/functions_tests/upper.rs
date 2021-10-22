use super::functions::{OneParamFunction, Upper};

#[test]
fn upper_test1() {
    assert_eq!(Upper::call(&vec!["templo".to_string()]), "TEMPLO");
}

#[test]
fn upper_test2() {
    assert_eq!(Upper::call(&vec!["TeMpLo".to_string()]), "TEMPLO");
}

#[test]
fn upper_test3() {
    assert_eq!(Upper::call(&vec!["TEMPLO".to_string()]), "TEMPLO");
}
