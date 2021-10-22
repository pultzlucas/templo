use super::functions::{OneParamFunction, UpperFirst};

#[test]
fn upper_first_test1() {
    assert_eq!(
        UpperFirst::call(&vec!["templo".to_string()]),
        "Templo".to_string()
    )
}

#[test]
fn upper_first_test2() {
    assert_eq!(
        UpperFirst::call(&vec!["BANANA".to_string()]),
        "BANANA".to_string()
    )
}

#[test]
fn upper_first_test3() {
    assert_eq!(
        UpperFirst::call(&vec!["Harry Potter".to_string()]),
        "Harry Potter".to_string()
    )
}

#[test]
fn upper_first_test4() {
    assert_eq!(
        UpperFirst::call(&vec!["hOBbit".to_string()]),
        "HOBbit".to_string()
    )
}
