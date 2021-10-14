mod cli;
mod core;
mod methods;

use crate::core::info::VERSION;
use crate::core::repos::Repository;
use crate::core::utils::errors::invalid_input_error;
use cli::input::command::parse_command;
use methods::*;
use std::env;

#[tokio::main]
async fn main() {
    let env: Vec<String> = env::args().collect();
    let command = parse_command(env).expect("Error when parsing command string.");

    if let None = command.method {
        if command.has_help_flag() {
            write_help!("../help_files/mod.json");
        }

        if command.has_version_flag() {
            println!("{}", VERSION);
        }

        if !command.has_help_flag() && !command.has_version_flag() && !command.args.is_empty() {
            eprintln!(
                "{}: {}",
                paint_string!("{red}", "Error"),
                format!("Invalid flag \"{}\"", &command.flags[0])
            )
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
                "gen" => Generate::run(command).await,
                "get" => Get::run(command).await,
                "del" => Delete::run(command),
                "regs" => Registry::run(command),
                "repo" => Repo::run(command),
                "repos" => Repos::run(command),
                "mv" => Move::run(command),
                "view" => View::run(command),
                "docs" => Docs::run(command),
                "save" => Save::run(command),
                "update" => Update::run(command),
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
