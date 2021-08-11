use crate::core::info::VERSION;
use std::io::Error;

pub fn version() -> Result<(), Error> {
    println!("{}", VERSION);
    Ok(())
}