use templo_engine::{Engine, EngineArg, EngineArgType};

// use super::config::ConfigArg;

// fn get_content_parser_config_args() -> Vec<ConfigArg> {
//     vec![
//         ConfigArg {
//             query: "asdasd".to_string(),
//             default: Some("value1".to_string()),
//             about: Some("Sets key1".to_string()),
//             key: "key1".to_string(),
//         },
//         ConfigArg {
//             query: "asdasd".to_string(),
//             default: None,
//             about: None,
//             key: "key2".to_string(),
//         },
//         ConfigArg {
//             query: "asdasd".to_string(),
//             default: Some("value3".to_string()),
//             about: None,
//             key: "key3".to_string(),
//         },
//     ]
// }

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

fn get_args() -> Vec<EngineArg> {
    vec![
        EngineArg {
            key: "id".to_string(),
            value: "123".to_string(),
            value_type: EngineArgType::String,
        },
        EngineArg {
            key: "thing".to_string(),
            value: "World!".to_string(),
            value_type: EngineArgType::String,
        },
        EngineArg {
            key: "name".to_string(),
            value: "Lucas".to_string(),
            value_type: EngineArgType::String,
        },
        EngineArg {
            key: "fn_name".to_string(),
            value: "add".to_string(),
            value_type: EngineArgType::String,
        },
        EngineArg {
            key: "folder1".to_string(),
            value: "folder1".to_string(),
            value_type: EngineArgType::String,
        },
    ]
}

#[test]
fn parse_template_content() {
    let engine = Engine::new(get_args());
    let parsed = engine.compile(TEXT.to_string()).unwrap();

    assert_eq!(
        parsed,
        r#"
function add(a, b) {
    return a + b
}

console.log("Hello World!")

const obj = {
    id: 123
    name: 'Lucas',
}
"#
    )
}