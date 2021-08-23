use crate::core::user_account::{logout_current, user_auth_exists};
use crate::utils::errors::not_found_error;
use std::io::Error;

pub fn run() -> Result<(), Error> {
    if !user_auth_exists() {
        return Err(not_found_error("You already is logged out."));
    }

    logout_current()?;
    println!("Logged out.");
    Ok(())
}
