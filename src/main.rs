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
    let mut args: &[String] = &[];

    if env.len() == 1 {
        prottern();
        return;
    }

    if env.len() > 2 {
        args = &env[2..];
    }

    let res = match env[1].as_str() {
        "init" => init(),
        "save" => save(args),
        "create" => create(args),
        "delete" => delete(args),
        "templates" => templates(),
        "describe" => describe(args),
        "register" => register().await,
        "login" => login().await,
        _ => {
            let err = Error::new(ErrorKind::InvalidInput, "Invalid command.");
            Err(err)
        }
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e)
    }
}
