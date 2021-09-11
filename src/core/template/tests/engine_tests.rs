use super::engine;

const TEXT: &'static str = r#"
function {> fn_name <}(a, b) {
    return a + b
}

console.log("Hello {> thing <}")
"#;

#[test]
fn parse_template_text() {
    let parsed = engine::parse(TEXT.to_string(), get_args()).unwrap();
    assert_eq!(parsed, r#"
function add(a, b) {
    return a + b
}

console.log("Hello World!")
"#)
}


fn get_args() -> Vec<engine::EngineArg> {
    vec![
        engine::EngineArg {
            key: "fn_name".to_string(),
            value: "add".to_string(),
        },
        engine::EngineArg {
            key: "thing".to_string(),
            value: "World!".to_string(),
        }
    ]
}