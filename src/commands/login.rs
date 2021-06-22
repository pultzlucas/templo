use crate::core::{
    io::{InputType, ProtternInput},
    user_account::{UserAccountKey, UserAccountManager},
};

use std::io::{Error, ErrorKind};

pub async fn login() -> Result<(), Error> {
    // Authenticating user account
    let response = {
        let (username, password) = (
            ProtternInput::get("Username: ", InputType::Text).unwrap(),
            ProtternInput::get("Password: ", InputType::Password).unwrap(),
        );
        UserAccountManager::authenticate_user_account(username, password).await
    };

    let user_account_auth: UserAccountKey = match response {
        Ok(response) => {
            if !response.authenticated {
                return Err(Error::new(ErrorKind::AlreadyExists, response.message));
            }

            serde_json::from_str(&response.user).unwrap()
        }
        Err(e) => return Err(e),
    };

    UserAccountManager::save_user_account(user_account_auth)?;

    println!("\nAccount was authenticated.");

    Ok(())
}
