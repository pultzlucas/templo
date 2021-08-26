use crate::utils::errors::{already_exists_error, invalid_input_error, other_error, std_error};
use crate::{
    cli::input::{get, get_valid_input, InputType},
    core::user_account::{
        request_account_confirmation, save_user_account, signup_user_account, check_user_signup, UserAccountData,
        UserAccountKey,
    },
    paintln,
};
use std::io::Error;
use std::time::Instant;

pub async fn run() -> Result<(), Error> {
    let account = {
        let inputs = (
            get("Username: ", InputType::Text)?,
            get("Email (this is public): ", InputType::Text)?,
            get("Password: ", InputType::Password)?,
            get("Confirm your password: ", InputType::Password)?,
        );

        let (username, email, password, _) = inputs;
        UserAccountData::new(username, email, password)
    };

    let start = Instant::now(); // start timing process


    // Check if user can be signup
    paintln!("{gray}", "[checking account credentials]");
    let user_can_be_signup = check_user_signup(&account).await?;
    if !user_can_be_signup.ok {
        return Err(invalid_input_error(&user_can_be_signup.message))
    }
    println!("Ok");

    // Send account confirmation token
    paintln!("{gray}", "[sending account confirmation]");
    let real_token = request_account_confirmation(account.username.clone(), account.email.clone()).await?;

    // Get valid token from user input
    println!("A confirmation token was sent to \"{}\" email.", account.email);
    get_valid_input("Enter your token here: ", InputType::Text, "Invalid token.", |input| {
        input == real_token
    })?;

    // Request user registration
    paintln!("{gray}", "[registering account]");
    let res = signup_user_account(account).await?;

    if !res.registered {
        return Err(already_exists_error(&res.message));
    }

    if let Some(user) = res.user {
        let user_account_registration: UserAccountKey = std_error(serde_json::from_str(&user))?;
        // Saving user account auth
        save_user_account(user_account_registration)?;
        println!("\nAccount was registered.");
        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));

        return Ok(());
    }

    Err(other_error("Something went wrong when signup."))
}

/* fn validate_signup_inputs(
    (username, email, password, password2): &RegisterFields,
) -> Result<(), Error> {
    let err = |msg: &str| Err(invalid_input_error(msg));

    if username.len() > 30 {
        return err("The username should not have more than 30 characters.");
    }

    if email.len() > 30 {
        return err("The email should not have more than 30 characters.");
    }

    validate_password(password, password2)?;

    Ok(())
} */

/* fn validate_password(password: &str, password2: &str) -> Result<(), Error> {
    let special_chars_regex = std_error(Regex::new(r"[^[a-z][A-Z]\d]"))?;
    let upper_chars_regex = std_error(Regex::new(r"[A-Z]"))?;
    let lower_chars_regex = std_error(Regex::new(r"[a-z]"))?;
    let digits_regex = std_error(Regex::new(r"[0-9]"))?;
    let whitespace_regex = std_error(Regex::new(r"\s"))?;

    if password.len() > 30 {
        return Err(invalid_input_error(
            "The password must have 30 characteres or less.",
        ));
    }
    if password.len() < 6 {
        return Err(invalid_input_error(
            "The password must have 6 characters or more.",
        ));
    }

    if !special_chars_regex.is_match(password) {
        return Err(invalid_input_error(
            "The password must have at least one special character.",
        ));
    }

    if !upper_chars_regex.is_match(password) {
        return Err(invalid_input_error(
            "The password must have at least one uppercase character.",
        ));
    }

    if !lower_chars_regex.is_match(password) {
        return Err(invalid_input_error(
            "The password must have at least one lowercase character.",
        ));
    }

    if !digits_regex.is_match(password) {
        return Err(invalid_input_error(
            "The password must have at least one digit.",
        ));
    }

    if whitespace_regex.is_match(password) {
        return Err(invalid_input_error(
            "The password must not have whitespaces.",
        ));
    }

    if password != password2 {
        return Err(invalid_input_error("The confirm password is incorrect."));
    }

    Ok(())
}
 */