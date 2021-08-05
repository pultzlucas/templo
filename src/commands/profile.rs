use crate::{
    cli::output::messages::error::NOT_FOUND_USER_AUTH, core::user_account::UserAccountManager,
};
use std::io::{Error, ErrorKind};

pub fn profile() -> Result<(), Error> {
    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }

    let current_user = UserAccountManager::get_user_account_data()?;
    println!("Name: {}", current_user.username);
    println!("Email: {}", current_user.email);

    Ok(())
}
