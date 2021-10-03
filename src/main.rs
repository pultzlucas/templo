extern crate tokio;
mod cli;
mod core;
mod methods;
mod utils;

use crate::utils::errors::invalid_input_error;
use cli::input::command::parse_command;
use methods::*;
use crate::core::repos::Repository;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let command = parse_command(env).expect("Error when parsing command string.");

    if let None = command.method {
        let flags = vec!["--help", "-h", "-H", "--version", "-v", "-V"];
        if let Err(err) = check_flags(&command.flags, flags) {
            eprintln!("{}: {}", paint_string!("{red}", "Error"), err)
        };

        if command.has_flag("--help") || command.has_flag("-h") || command.has_flag("-H") {
            if let Err(err) = help::run() {
                eprintln!("{}: {}", paint_string!("{red}", "Error"), err)
            };
        }

        if command.has_flag("--version") || command.has_flag("-v") || command.has_flag("-V") {
            if let Err(err) = version::run() {
                eprintln!("{}: {}", paint_string!("{red}", "Error"), err)
            };
        }

        if command.flags.is_empty() {
            welcome::run();
        }

        return;
    }

    if let Err(err) = Repository::create("main") {
        eprintln!("{}: {}", paint_string!("{red}", "Error"), err)
    }

    if let Some(method) = command.method.clone() {
        let output = {
            match method.as_str() {
                "gen" => generate::run(command).await,
                "get" => get::run(command).await,
                "del" => delete::run(command),
                "namespace" => namespace::run(command),
                "repo" => repo::run(command),
                "repos" => repos::run(command),
                "mv" => r#move::run(command),
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
