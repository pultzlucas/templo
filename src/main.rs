mod commands;
mod utils;
mod core;

use commands::*;
use std::env;

#[allow(unused_variables, unused_assignments)]
fn main() {
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
        "delete" => delete(args),
        "templates" => templates(),
        "describe" => describe(args),
        _ => Err("Invalid command.".to_string())
    };

    match res {
        Ok(o) => println!("{}", o),
        Err(e) => println!("Error: {}", e)
    }
}
