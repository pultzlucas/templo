use super::functions::{Join, TwoParamFunction};

#[test]
fn join_test1() {
    assert_eq!(
        Join::call(&vec!["templo tool".to_string(), " ".to_string()]),
        "templotool"
    );
}

#[test]
fn join_test2() {
    assert_eq!(
        Join::call(&vec!["templo tool".to_string(), "".to_string()]),
        "templotool"
    );
}

#[test]
fn join_test3() {
    assert_eq!(
        Join::call(&vec!["TEMPLO_TOOL".to_string(), "_".to_string()]),
        "TEMPLOTOOL"
    );
}

#[test]
fn join_test4() {
    assert_eq!(
        Join::call(&vec!["templo   tool".to_string(), "   ".to_string()]),
        "templotool"
    );
}
