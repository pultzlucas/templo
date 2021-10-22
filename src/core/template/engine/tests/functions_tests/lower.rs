use super::functions::{Lower, OneParamFunction};

#[test]
fn lower_test1() {
    assert_eq!(Lower::call(&vec!["templo".to_string()]), "templo");
}

#[test]
fn lower_test2() {
    assert_eq!(Lower::call(&vec!["TeMpLo".to_string()]), "templo");
}

#[test]
fn lower_test3() {
    assert_eq!(Lower::call(&vec!["TEMPLO".to_string()]), "templo");
}
