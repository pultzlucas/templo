use std::io::Error;
use crate::core::user_account::UserAccountManager;

pub fn profile() -> Result<(), Error> {
    let current_user = match UserAccountManager::get_user_account_data() {
        Err(e) => return Err(e),
        Ok(o) => o,
    };
    
    println!("Name: {}", current_user.username);
    println!("Email: {}", current_user.email);

    Ok(())
}