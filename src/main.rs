extern crate tokio;
mod cli;
mod methods;
mod core;
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
                "save" => save::run(args),
                "gen" => generate::run(args),
                "del" => delete::run(args),
                "update" => update::run(args),
                "repo" => repo::run(args),
                "desc" => describe::run(args),
                "help" | "h" => help::run(),
                "version" | "v" => version::run(),
                "docs" => documentation::run(),
                "reg" => registry::run(args),
                "get" => get::run(args).await,
                _ => Err(invalid_input_error(&format!(
                    "Invalid command \"{}\".",
                    command
                ))),
            }
        };

        if let Err(e) = output {
            eprintln!("{}: {}", paint_string!("{red}", "Error"), e);
        }
    }
}
