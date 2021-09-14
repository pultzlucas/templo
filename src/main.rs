extern crate tokio;
mod cli;
mod core;
mod methods;
mod utils;

use crate::utils::errors::invalid_input_error;
use cli::input::{args::parse_args, command::parse_command};
use methods::*;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let command = parse_command(env.join(" ").clone()).expect("Error when parsing command string.");
    let args = parse_args(env.join(" ")).expect("Error when parsing command args.");

    if let None = command.method {
        welcome::run();
        return;
    }

    if let Some(method) = command.method.clone() {
        let output = {
            match method.as_str() {
                "gen" => generate::run(args),
                "del" => delete::run(args),
                "namespace" => namespace::run(args),
                "get" => get::run(args).await,
                "repo" => repo::run(command),
                "desc" => describe::run(args),
                "docs" => docs::run(),
                "save" => save::run(args),
                "update" => update::run(args),
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
