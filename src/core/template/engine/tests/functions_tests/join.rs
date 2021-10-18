use super::functions::join;

#[test]
fn join_test1() {
    assert_eq!(join("templo tool", Some(" ")), "templotool");
}

#[test]
fn join_test2() {
    assert_eq!(join("templo tool", None), "templotool");
}

#[test]
fn join_test3() {
    assert_eq!(join("TEMPLO_TOOL", Some("_")), "TEMPLOTOOL");
}

#[test]
fn join_test4() {
    assert_eq!(join("templo   tool", Some("   ")), "templotool");
}

