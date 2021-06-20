use crate::core::info::{REPOSITORY, VERSION};
use super::COMMANDS;
use std::io::Error;

pub fn help() -> Result<(), Error> {
    println!("Version:\n{}v\n", VERSION);
    println!("Respository:\n{}\n", REPOSITORY);
    println!("Commands:");
    print_commands();

    Ok(())
}

fn print_commands() {
    for (name, description) in COMMANDS.iter() {
        println!("{:>9}\t{}", name, description);
    }
}
