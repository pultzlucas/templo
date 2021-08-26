extern crate rpassword;
pub mod args;

use rpassword::read_password;
use std::io::{stdin, stdout, Error, Write};

#[derive(Clone)]
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

pub fn get_valid_input<F: Fn(&str) -> bool>(
    text: &str,
    input_type: InputType,
    invalid_input_msg: &str,
    check_input: F,
) -> Result<String, Error> {
    let input = get(text, input_type.clone())?;

    if check_input(&input) {
        return Ok(input);
    }

    println!("{}", invalid_input_msg);
    get_valid_input(text, input_type, invalid_input_msg, check_input)
}
