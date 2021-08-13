use crate::core::utils::errors::{already_exists_error, other_error, std_error};
use crate::{
    cli::input::{get, InputType},
    core::user_account::{save_user_account, signup_user_account, UserAccountData, UserAccountKey},
    paintln,
};
use std::io::{Error, ErrorKind};
use std::time::Instant;

type RegisterFields = (String, String, String, String);

pub async fn signup() -> Result<(), Error> {
    let user_account = {
        let inputs = (
            get("Username: ", InputType::Text)?,
            get("Email (this is public): ", InputType::Text)?,
            get("Password: ", InputType::Password)?,
            get("Confirm your password: ", InputType::Password)?,
        );

        validate_signup_inputs(&inputs)?;

        let (username, email, password, _) = inputs;
        UserAccountData::new(username, email, password)
    };

    // Requesting registration
    let start = Instant::now(); // start timing process

    paintln!("{gray}", "[Registering Account]");
    let res = signup_user_account(&user_account).await?;

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

fn validate_signup_inputs(
    (username, email, password, password2): &RegisterFields,
) -> Result<(), Error> {
    let err = |msg: &str| Err(Error::new(ErrorKind::InvalidInput, msg));

    if username.len() > 30 {
        return err("The username should not have more than 30 characters.");
    }

    if email.len() > 30 {
        return err("The email should not have more than 30 characters.");
    }
    if password.len() > 30 {
        return err("The password should not have more than 30 characters.");
    }

    if password != password2 {
        return err("The confirm password is incorrect.");
    }

    Ok(())
}
