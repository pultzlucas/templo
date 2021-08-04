use crate::{
    core::{
        io::{InputType, ProtternInput},
        user_account::{UserAccountData, UserAccountKey, UserAccountManager},
    },
    paintln,
};
use std::io::{Error, ErrorKind};
use std::time::Instant;

type RegisterFields = (String, String, String, String);

pub async fn signup() -> Result<(), Error> {
    let user_account = {
        let inputs = (
            ProtternInput::get("Username: ", InputType::Text).unwrap(),
            ProtternInput::get("Email (this is public): ", InputType::Text).unwrap(),
            ProtternInput::get("Password: ", InputType::Password).unwrap(),
            ProtternInput::get("Confirm your password: ", InputType::Password).unwrap(),
        );

        validate_signup_inputs(&inputs)?;

        let (username, email, password, _) = inputs;
        UserAccountData::new(username, email, password)
    };

    // Requesting registration
    let start = Instant::now(); // start timing process

    paintln!("{gray}", "[Registering Account]");
    let res = UserAccountManager::signup_user_account(&user_account).await?;
    if !res.registered {
        return Err(Error::new(ErrorKind::AlreadyExists, res.message));
    }

    let user_account_registration: UserAccountKey = serde_json::from_str(&res.user).unwrap();

    // Saving user account auth
    UserAccountManager::save_user_account(user_account_registration)?;
    println!("\nAccount was registered.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
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
