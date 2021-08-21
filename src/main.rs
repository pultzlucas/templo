extern crate tokio;
mod cli;
mod commands;
mod core;
mod utils;

use crate::utils::errors::invalid_input_error;
use cli::input::args::parse_args;
use commands::*;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let args = parse_args(env.join(" ")).expect("Error when parsing command args.");

    if let None = args.command {
        prottern::run();
        return;
    }

    if let Some(command) = args.command.clone() {
        let output = {
            match command.as_str() {
                "save" => save::run(args),
                "generate" | "gen" => generate::run(args),
                "delete" | "del" => delete::run(args),
                "repo" | "repository" => repository::run(args),
                "describe" | "desc" => describe::run(args),
                "profile" => profile::run(),
                "help" | "h" => help::run(),
                "version" | "v" => version::run(),
                "logout" => logout::run(),
                "documentation" | "docs" => documentation::run(),
                "signup" => signup::run().await,
                "login" => login::run().await,
                "pub" => publish::run(args).await,
                "unpub" => unpub::run(args).await,
                "get" => get::run(args).await,
                "explore" => explore::run().await,
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
