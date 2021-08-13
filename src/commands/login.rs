use crate::utils::errors::already_exists_error;
use crate::{
    cli::input::{get, InputType},
    core::user_account::{authenticate_user_account, save_user_account},
    paintln,
};

use std::io::Error;
use std::time::Instant;

pub async fn login() -> Result<(), Error> {
    let (username, password) = (
        get("Username: ", InputType::Text).unwrap(),
        get("Password: ", InputType::Password).unwrap(),
    );

    // Making authentication
    let start = Instant::now(); //start timing process
    let response = {
        paintln!("{gray}", "[Authenticating Account]");
        let response = authenticate_user_account(username, password).await?;
        response
    };

    if !response.authenticated {
        return Err(already_exists_error(&response.message));
    }

    let user_account_auth =
        serde_json::from_str(&response.user).expect("Error when parsing user account object.");

    save_user_account(user_account_auth)?;
    println!("\nAccount was authenticated.");
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
