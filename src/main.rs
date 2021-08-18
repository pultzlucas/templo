extern crate tokio;
mod cli;
mod commands;
mod core;
mod utils;

use commands::*;
use crate::core::user_account::{get_user_account_data, user_auth_exists};
use crate::utils::errors::invalid_input_error;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let args: &[String] = if env.len() > 2 { &env[2..] } else { &[] };

    
    if env.len() == 1 {
        prottern::run();
        return;
    }

    //print current user account before execute other commands
    if user_auth_exists() {
        if let Ok(user) = get_user_account_data() {
            paintln!("{gray}", user.username);
        }
    }

    let output = {
        let command = env[1].as_str();
        match command {
            "save" => save::run(args),
            "generate" | "gen" => generate::run(args),
            "delete" | "del" => delete::run(args),
            "repo" | "repository" => repository::run(),
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
            _ => Err(invalid_input_error(&format!("Invalid command \"{}\".", command))),
        }
    };

    if let Err(e) = output {
        eprintln!("{}: {}", paint_string!("{red}", "Error"), e);
    }
}
