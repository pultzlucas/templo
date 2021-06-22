use std::io::{Error, ErrorKind};
use crate::core::user_account::UserAccountManager;

pub fn profile() -> Result<(), Error> {
    if !UserAccountManager::user_auth_exists() {
        let err = Error::new(
            ErrorKind::NotFound, 
            r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern register" to register one.
If you already have a user account created, type "prottern login" to authenticate it."#
        );
        return Err(err);
    }

    let current_user = UserAccountManager::get_user_account_data()?;
    
    println!("Name: {}", current_user.username);
    println!("Email: {}", current_user.email);

    Ok(())
}