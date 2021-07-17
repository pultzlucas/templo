use crate::core::user_account::UserAccountManager;
use std::io::Error;

pub fn logout() -> Result<(), Error>{
    UserAccountManager::logout_current()?;
    println!("Logged out.");
    Ok(())
}