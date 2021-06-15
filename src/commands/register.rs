use crate::core::{
    io::ProtternInput,
    user_account::{UserAccountData, UserAccountManager},
};
use std::io::{Error, ErrorKind};

type RegisterFields = (String, String, String, String);

pub async fn register() -> Result<(), Error> {
    let inputs = (
        ProtternInput::get("Username: ").unwrap(),
        ProtternInput::get("Email: ").unwrap(),
        ProtternInput::get("Password: ").unwrap(),
        ProtternInput::get("Confirm your passoword: ").unwrap(),
    );

    if let Err(e) = validate_register_inputs(&inputs) {
        return Err(e);
    }

    let (username, email, password, _) = inputs;
    let user_account = UserAccountData::new(username, email, password);

    // Requesting registration
    let res = UserAccountManager::register_user_account(&user_account).await;
    let key = match res {
        Ok(res) => {
            if !res.registered {
                let err = Error::new(ErrorKind::AlreadyExists, res.message);
                return Err(err);
            }

            res.key
        }
        Err(e) => return Err(e),
    };

    // Saving user account authentication

    UserAccountManager::log_user_account(user_account, key)?;

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
