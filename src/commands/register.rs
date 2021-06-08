use crate::core::UserAccountManager;
use crate::utils::structs::UserAccountData;
use std::io::{stdin, stdout, Error, ErrorKind, Write};

type RegisterFields = (String, String, String, String);

pub fn register<'a>() -> Result<&'a str, Error> {
    let fields = (
        ask_field("Username: ")?,
        ask_field("Email: ")?,
        ask_field("Password: ")?,
        ask_field("Confirm your passoword: ")?,
    );

    if let Err(e) = validate_register_fields(&fields) {
        eprintln!("Error: {}\n", e);
        register()?;
    }

    // save the user account file

    let (username, email, password, _) = fields;

    UserAccountManager::create_user_account(&UserAccountData {
        username,
        email,
        password,
    })?;

    Ok("")
}

fn validate_register_fields(
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

fn ask_field(text: &str) -> Result<String, Error> {
    print!("{}", text);
    stdout().flush().unwrap();

    let mut info = String::new();
    if let Err(e) = stdin().read_line(&mut info) {
        return Err(e);
    }

    Ok(info.trim().to_string())
}
