extern crate tokio;
mod commands;
mod core;

use commands::*;
use std::{
    env,
    io::{Error, ErrorKind},
};

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
            "create" => create(args),
            "delete" => delete(args),
            "templates" => templates(),
            "describe" => describe(args),
            "profile" => profile(),
            "help" => help(),
            "version" => version(),
            "logout" => logout(),
            "signup" => signup().await,
            "login" => login().await,
            "pub" => publish(args).await,
            "unpub" => unpub(args).await,
            "get" => get(args).await,
            "discover" => discover().await,
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid command.")),
        }
    };

    if let Err(e) = output {
        eprintln!("{}: {}", paint_string!("{red}", "Error"), e);
    }
}
