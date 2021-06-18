use crate::core::{
    io::{ProtternInput, InputType},
    user_account::{UserAccountKey, UserAccountManager},
};

use std::io::{Error, ErrorKind};

pub async fn login() -> Result<(), Error> {
    let (username, password) = (
        ProtternInput::get("Username: ", InputType::Text).expect("Error when get login data."),
        ProtternInput::get("Password: ", InputType::Password).expect("Error when get login data."),
    );
    // Authenticating user account
    let res = UserAccountManager::authenticate_user_account(username, password).await;
    let user_account_auth: UserAccountKey = match res {
        Ok(res) => {
            if !res.authenticated {
                let err = Error::new(ErrorKind::AlreadyExists, res.message);
                return Err(err);
            }

            serde_json::from_str(&res.user).unwrap()
        }
        Err(e) => return Err(e),
    };

    UserAccountManager::save_user_account(user_account_auth)?;

    println!("\nAccount was authenticated.");

    Ok(())
}
