use crate::{
    core::{
        io::{InputType, ProtternInput},
        user_account::UserAccountManager,
    },
    paintln,
};

use std::io::{Error, ErrorKind};

pub async fn login() -> Result<(), Error> {
    let response = {
        let (username, password) = (
            ProtternInput::get("Username: ", InputType::Text).unwrap(),
            ProtternInput::get("Password: ", InputType::Password).unwrap(),
        );
        paintln!("{gray}", "[Authenticating Account]");
        UserAccountManager::authenticate_user_account(username, password).await?
    };

    if !response.authenticated {
        return Err(Error::new(ErrorKind::AlreadyExists, response.message));
    }

    let user_account_auth =
        serde_json::from_str(&response.user).expect("Error when parsing user account object.");

    UserAccountManager::save_user_account(user_account_auth)?;

    println!("\nAccount was authenticated.");

    Ok(())
}
