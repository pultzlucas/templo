use std::io::Error;
use crate::cli::input::args::Args;

pub fn run(args: Args) -> Result<(), Error> {
    println!("config");
    Ok(())
}