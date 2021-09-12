extern crate tokio;
mod cli;
mod core;
mod methods;
mod utils;

use crate::utils::errors::invalid_input_error;
use cli::input::args::parse_args;
use methods::*;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let args = parse_args(env.join(" ")).expect("Error when parsing command args.");

    if let None = args.command {
        welcome::run();
        return;
    }

    if let Some(command) = args.command.clone() {
        let output = {
            match command.as_str() {
                "gen" => generate::run(args),
                "del" => delete::run(args),
                "reg" => namespace::run(args),
                "get" => get::run(args).await,
                "repo" => repo::run(args),
                "desc" => describe::run(args),
                "docs" => docs::run(),
                "save" => save::run(args),
                "update" => update::run(args),
                "help" | "h" => help::run(),
                "version" | "v" => version::run(),
                _ => Err(invalid_input_error(&format!(
                    "Invalid method \"{}\".",
                    command
                ))),
            }
        };

        if let Err(e) = output {
            eprintln!("{}: {}", paint_string!("{red}", "Error"), e);
        }
    }
}
