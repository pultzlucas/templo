use crate::core::{
    io::{InputType, ProtternInput},
    user_account::{UserAccountData, UserAccountKey, UserAccountManager},
};
use std::io::{Error, ErrorKind};

type RegisterFields = (String, String, String, String);

pub async fn register() -> Result<(), Error> {
    let inputs = (
        ProtternInput::get("Username: ", InputType::Text).ok(),
        ProtternInput::get("Email: ", InputType::Text).ok(),
        ProtternInput::get("Password: ", InputType::Password).ok(),
        ProtternInput::get("Confirm your password: ", InputType::Password).ok(),
    );

    if let Err(e) = validate_register_inputs(&inputs) {
        return Err(e);
    }

    let (username, email, password, _) = inputs;
    let user_account = UserAccountData::new(username, email, password);

    // Requesting registration
    let res = UserAccountManager::register_user_account(&user_account).await;
    let user_account_registration: UserAccountKey = match res {
        Ok(res) => {
            if !res.registered {
                let err = Error::new(ErrorKind::AlreadyExists, res.message);
                return Err(err);
            }

            serde_json::from_str(&res.user).unwrap()
        }
        Err(e) => return Err(e),
    };

    // Saving user account authentication

    UserAccountManager::save_user_account(user_account_registration)?;

    println!("\nAccount was registered.");

    Ok(())
}

fn validate_register_inputs(
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
