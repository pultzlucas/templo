use super::functions::upper_first;

#[test]
fn upper_first_test1() {
    assert_eq!(upper_first("templo"), "Templo".to_string())
}

#[test]
fn upper_first_test2() {
    assert_eq!(upper_first("BANANA"), "BANANA".to_string())
}

#[test]
fn upper_first_test3() {
    assert_eq!(upper_first("Harry Potter"), "Harry Potter".to_string())
}

#[test]
fn upper_first_test4() {
    assert_eq!(upper_first("hOBbit"), "HOBbit".to_string())
}