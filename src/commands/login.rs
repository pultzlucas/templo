use crate::core::{
    io::ProtternInput,
    user_account::{UserAccountData, UserAccountManager},
};

use std::io::{Error, ErrorKind};

pub async fn login() -> Result<(), Error> {
    let (username, password) = (
        ProtternInput::get("Username: ").unwrap(),
        ProtternInput::get("Password: ").unwrap(),
    );
    // Authenticating user account

    let user_account = UserAccountData::new(username, "".to_string(), password);
    let res = UserAccountManager::authenticate_user_account(&user_account).await;
    let key = match res {
        Ok(res) => {
            if !res.authenticated {
                let err = Error::new(ErrorKind::AlreadyExists, res.message);
                return Err(err);
            }

            res.key
        }
        Err(e) => return Err(e),
    };

    UserAccountManager::log_user_account(user_account, key)?;

    println!("\nAccount was authenticated.");

    Ok(())
}
