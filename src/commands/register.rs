use std::io::{stdin, stdout, Error, ErrorKind, Write};
use crate::utils::structs::RegisterFields;
use crate::core::UserAccountManager;

pub fn register<'a>() -> Result<&'a str, Error> {
    let register_fields = RegisterFields {
        username: ask_field("Username: ")?,
        email: ask_field("Email: ")?,
        password: ask_field("Password: ")?,
        password2: ask_field("Confirm your passoword: ")?,
    };

    if let Err(e) = validate_register_fields(&register_fields) {
        eprintln!("Error: {}\n", e);
        register()?;
    }

    // save the user account file

    UserAccountManager::create_user_account(&register_fields);

    Ok("")
}

fn validate_register_fields(fields: &RegisterFields) -> Result<(), Error> {
    let err = |msg: &str| Err(Error::new(ErrorKind::InvalidInput, msg));

    if fields.username.len() > 30 {
        return err("The username should not have more than 30 characters.");
    }

    if fields.email.len() > 30 {
        return err("The email should not have more than 30 characters.");
    }
    if fields.password.len() > 30 {
        return err("The password should not have more than 30 characters.");
    }

    if fields.password != fields.password2 {
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
