use super::functions::upper;

#[test]
fn upper_test1() {
    assert_eq!(upper("templo"), "TEMPLO");
}

#[test]
fn upper_test2() {
    assert_eq!(upper("TeMpLo"), "TEMPLO");
}

#[test]
fn upper_test3() {
    assert_eq!(upper("TEMPLO"), "TEMPLO");
}