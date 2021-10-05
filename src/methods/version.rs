use crate::core::info::VERSION;
use std::io::Error;

pub struct Version;

impl Version {
    pub fn run() -> Result<(), Error> {
        println!("{}", VERSION);
        Ok(())
    }
}
