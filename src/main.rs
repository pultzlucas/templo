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
    let command = parse_command(env).expect("Error when parsing command string.");

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