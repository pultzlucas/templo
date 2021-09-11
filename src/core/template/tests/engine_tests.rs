use super::engine::{parse, TempEngineArg};

const TEXT: &'static str = r#"
function {> fn_name <}(a, b) {
    return a + b
}

console.log("Hello {> thing <}")

const obj = {
    id: {> id <}
    name: '{> name <}',
}
"#;

fn get_args() -> Vec<TempEngineArg> {
    vec![
        TempEngineArg {
            key: "id".to_string(),
            value: "123".to_string(),
        },
        TempEngineArg {
            key: "thing".to_string(),
            value: "World!".to_string(),
        },
        TempEngineArg {
            key: "name".to_string(),
            value: "Lucas".to_string(),
        },
        TempEngineArg {
            key: "fn_name".to_string(),
            value: "add".to_string(),
        },
    ]
}

#[test]
fn parse_template_text() {
    let parsed = parse(TEXT.to_string(), get_args()).unwrap();

    assert_eq!(parsed, r#"
function add(a, b) {
    return a + b
}

console.log("Hello World!")

const obj = {
    id: 123
    name: 'Lucas',
}
"#)
}

