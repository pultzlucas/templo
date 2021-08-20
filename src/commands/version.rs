use crate::core::info::VERSION;
use std::io::Error;

pub fn run() -> Result<(), Error> {
    println!("{}", VERSION);
    Ok(())
}
