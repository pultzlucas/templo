use super::config::{get_config_args, ConfigArgs};

#[test]
fn it_should_get_config_args_from_dir() {
    let config_args = get_config_args("./folder-for-tests").unwrap();

    assert_eq!(
        config_args,
        vec![
            ConfigArgs {
                key: "name".to_string(),
                query: "Your name (pultzlucas): ".to_string(),
                default: Some("pultzlucas".to_string())
            },
            ConfigArgs {
                key: "lastName".to_string(),
                query: "Your last name: ".to_string(),
                default: None
            }
        ]
    )
}
