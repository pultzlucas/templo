extern crate tokio;
mod cli;
mod commands;
mod core;
mod utils;

use crate::utils::errors::invalid_input_error;
use cli::input::parse_args;
use commands::*;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let args: &[String] = if env.len() > 2 { &env[2..] } else { &[] };
    let args2 = parse_args(env.join(" ")).expect("Error when parsing command args.");

    println!("{:?}", args2);

    if env.len() == 1 {
        prottern::run();
        return;
    }

    let output = {
        let command = env[1].as_str();
        match command {
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
