extern crate rpassword;
pub mod args;

use rpassword::read_password;
use std::io::{stdin, stdout, Error, Write};

pub enum InputType {
    Text,
    Password,
}

pub fn get(text: &str, input_type: InputType) -> Result<String, Error> {
    print!("{}", text);
    stdout().flush()?;
    match input_type {
        InputType::Text => {
            let mut info = String::new();
            stdin().read_line(&mut info)?;
            Ok(info.trim().to_string())
        }

        InputType::Password => Ok(read_password()?.trim().to_string()),
    }
}
