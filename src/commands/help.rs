use super::COMMANDS;
use std::io::Error;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");

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
