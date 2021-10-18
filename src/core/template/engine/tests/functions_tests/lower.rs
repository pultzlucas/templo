use super::functions::lower;

#[test]
fn lower_test1() {
    assert_eq!(lower("templo"), "templo");
}

#[test]
fn lower_test2() {
    assert_eq!(lower("TeMpLo"), "templo");
}

#[test]
fn lower_test3() {
    assert_eq!(lower("TEMPLO"), "templo");
}