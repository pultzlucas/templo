mod commands;
use commands::prottern;
use std::env;

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

    match &env[1] {
        _ => println!("Invalid command."),
    }
}
