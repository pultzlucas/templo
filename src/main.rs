extern crate tokio;
mod cli;
mod commands;
mod core;
mod utils;

use commands::*;
use crate::utils::errors::invalid_input_error;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let args: &[String] = if env.len() > 2 { &env[2..] } else { &[] };

    if env.len() == 1 {
        prottern();
        return;
    }

    let output = {
        let command = env[1].as_str();
        match command {
            "save" => save(args),
            "generate" | "gen" => generate(args),
            "delete" | "del" => delete(args),
            "repo" | "repository" => repository(),
            "describe" | "desc" => describe(args),
            "profile" => profile(),
            "help" | "h" => help(),
            "version" | "v" => version(),
            "logout" => logout(),
            "documentation" | "docs" => documentation(),
            "signup" => signup().await,
            "login" => login().await,
            "pub" => publish(args).await,
            "unpub" => unpub(args).await,
            "get" => get(args).await,
            "explore" => explore().await,
            _ => Err(invalid_input_error(&format!("Invalid command \"{}\".", command))),
        }
    };

    if let Err(e) = output {
        eprintln!("{}: {}", paint_string!("{red}", "Error"), e);
    }
}
