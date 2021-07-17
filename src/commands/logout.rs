use crate::core::user_account::UserAccountManager;
use std::io::{Error, ErrorKind};

pub fn logout() -> Result<(), Error> {
    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "You already is logged out.",
        ));
    }

    UserAccountManager::logout_current()?;
    println!("Logged out.");
    Ok(())
}
