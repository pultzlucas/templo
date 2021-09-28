extern crate tokio;
mod cli;
mod core;
mod methods;
mod utils;

use crate::utils::errors::invalid_input_error;
use cli::input::command::parse_command;
use methods::*;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    println!("{:?}", env);
    let command = parse_command(env).expect("Error when parsing command string.");

    println!("{:?}", command);

    if let None = command.method {
        welcome::run();
        return;
    }

    if let Some(method) = command.method.clone() {
        let output = {
            match method.as_str() {
                "gen" => generate::run(command),
                "del" => delete::run(command),
                "namespace" => namespace::run(command),
                "get" => get::run(command).await,
                "repo" => repo::run(command),
                "desc" => describe::run(command),
                "docs" => docs::run(),
                "save" => save::run(command),
                "update" => update::run(command),
                "help" | "h" => help::run(),
                "version" | "v" => version::run(),
                _ => Err(invalid_input_error(&format!(
                    "Invalid method \"{}\".",
                    method
                ))),
            }
        };

        if let Err(e) = output {
            eprintln!("{}: {}", paint_string!("{red}", "Error"), e);
        }
    }
}

// #[test]
// fn it_should_return_a_valid_command_struct() {
//     //let command = r#"tp method  submethod  arg1 -f --flag1 ./arg2  --flag-2  arg-3 --option1=value1 arg_4 --option-2=value-2 --empty-option= --option_quotes="A text inside \"of\" double quotes.""#.to_string();
//     let struct_tested = parse_command(args).unwrap();

//     let correct_struct = Command {
//         method: Some("method".to_string()),
//         submethod: Some("submethod".to_string()),
//         flags: vec![
//             "-f".to_string(),
//             "--flag1".to_string(),
//             "--flag-2".to_string(),
//         ],
//         options: vec![
//             CommandOption {
//                 name: "option1".to_string(),
//                 value: "value1".to_string(),
//             },
//             CommandOption {
//                 name: "option-2".to_string(),
//                 value: "value-2".to_string(),
//             },
//             CommandOption {
//                 name: "empty-option".to_string(),
//                 value: "".to_string(),
//             },
//             CommandOption {
//                 name: "option_quotes".to_string(),
//                 value: "A text inside of double quotes.".to_string(),
//             },
//         ],
//         args: vec![
//             "submethod".to_string(),
//             "arg1".to_string(),
//             "./arg2".to_string(),
//             "arg-3".to_string(),
//             "arg_4".to_string(),
//         ],
//     };

//     assert_eq!(correct_struct, struct_tested)
// }