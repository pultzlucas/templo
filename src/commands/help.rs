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
    for command_info in COMMANDS.iter() {
        println!("{:>20}\t{}", command_info.name, command_info.description);
    }
}
