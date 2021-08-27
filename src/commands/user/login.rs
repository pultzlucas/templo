use crate::utils::errors::{already_exists_error, invalid_input_error};
use crate::{
    cli::input::{get, get_valid_input, InputType},
    core::user_account::{log_user_account, request_account_confirmation, save_user_account, check_user_login},
    paintln,
};

use std::io::Error;
use std::time::Instant;

pub async fn run() -> Result<(), Error> {
    let (username, password) = (
        get("Username: ", InputType::Text).unwrap(),
        get("Password: ", InputType::Password).unwrap(),
    );

    let start = Instant::now(); //start timing process

    // Check if user can be logged
    paintln!("{gray}", "[checking account credentials]");
    let user_can_be_logged = check_user_login(username.clone(), password.clone()).await?;
    if !user_can_be_logged.ok {
        return Err(invalid_input_error(&user_can_be_logged.message));
    }
    println!("Ok");

    // Send account confirmation token
    paintln!("{gray}", "[sending account confirmation]");
    let (real_token, email) = request_account_confirmation(username.clone(), None, true).await?;

    // Get valid token from user input
    println!(
        "A confirmation token was sent to \"{}\" email.",
        email
    );
    get_valid_input(
        "Enter your token here: ",
        InputType::Text,
        "Invalid token.",
        |input| input == real_token,
    )?;

    // Request user login
    let response = {
        paintln!("{gray}", "[authenticating account]");
        let response = log_user_account(username, password).await?;
        response
    };

    if !response.logged {
        return Err(already_exists_error(&response.message));
    }

    let user_account_auth =
        serde_json::from_str(&response.user).expect("Error when parsing user account object.");

    save_user_account(user_account_auth)?;
    println!("\nAccount was logged.");
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
