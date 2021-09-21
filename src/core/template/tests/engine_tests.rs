use super::{
    config::ConfigArg,
    engine::{parse_content, set_arg_default_value, TempEngineArg},
};

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
fn parse_template_content() {
    let parsed = parse_content(TEXT.to_string(), get_args()).unwrap();

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

fn get_config_args() -> Vec<ConfigArg> {
    vec![
        ConfigArg {
            query: "asdasd".to_string(),
            default: Some("value1".to_string()),
            key: "key1".to_string(),
        },
        ConfigArg {
            query: "asdasd".to_string(),
            default: None,
            key: "key2".to_string(),
        },
        ConfigArg {
            query: "asdasd".to_string(),
            default: Some("value3".to_string()),
            key: "key3".to_string(),
        },
    ]
}

#[test]
fn set_default_value_in_engine_arg1() {
    let engine_arg = TempEngineArg {
        key: "key1".to_string(),
        value: "".to_string(),
    };

    assert_eq!(
        set_arg_default_value(engine_arg, &get_config_args()).unwrap(),
        TempEngineArg {
            key: "key1".to_string(),
            value: "value1".to_string()
        }
    );
}

#[test]
fn no_set_none_default_value_in_engine_arg2() {
    let engine_arg = TempEngineArg {
        key: "key2".to_string(),
        value: "".to_string(),
    };

    assert_eq!(
        set_arg_default_value(engine_arg, &get_config_args()).unwrap(),
        TempEngineArg {
            key: "key2".to_string(),
            value: "".to_string()
        }
    );
}

#[test]
fn no_set_default_value_in_no_empty_engine_arg2() {
    let engine_arg = TempEngineArg {
        key: "key3".to_string(),
        value: "value3".to_string(),
    };

    assert_eq!(
        set_arg_default_value(engine_arg, &get_config_args()).unwrap(),
        TempEngineArg {
            key: "key3".to_string(),
            value: "value3".to_string()
        }
    );
}
