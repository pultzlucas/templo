use std::io::{Error, ErrorKind};
use crate::core::{
    requester::{Method, ProtternRequester, RegisterResponse},
    user_account::{UserAccountData, UserAccountManager},
    io::ProtternInput
};

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

    // Requesting authentication

    let body = serde_json::to_string(&user_account).unwrap();
    let response = ProtternRequester::request("/user/register", Method::POST, body)
        .await
        .unwrap();
    let response_json: RegisterResponse = serde_json::from_str(&response).unwrap();

    if !response_json.registered {
        let err = Error::new(ErrorKind::AlreadyExists, response_json.message);
        return Err(err);
    }

    // Saving user account authentication

    UserAccountManager::log_user_account(user_account, response_json.key)?;

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
