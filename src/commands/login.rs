use crate::{
    core::{
        io::{InputType, ProtternInput},
        user_account::UserAccountManager,
    },
    paintln,
};

use std::io::{Error, ErrorKind};
use std::time::Instant;

pub async fn login() -> Result<(), Error> {
    let (username, password) = (
        ProtternInput::get("Username: ", InputType::Text).unwrap(),
        ProtternInput::get("Password: ", InputType::Password).unwrap(),
    );

    // Making authentication
    let start = Instant::now(); //start timing process
    let response = {
        paintln!("{gray}", "[Authenticating Account]");
        let response = UserAccountManager::authenticate_user_account(username, password).await?;
        response
    };

    if !response.authenticated {
        return Err(Error::new(ErrorKind::AlreadyExists, response.message));
    }

    let user_account_auth =
        serde_json::from_str(&response.user).expect("Error when parsing user account object.");

    UserAccountManager::save_user_account(user_account_auth)?;
    println!("\nAccount was authenticated.");
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
