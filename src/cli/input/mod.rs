pub mod command;

use self::command::Command;
use crate::utils::errors::invalid_input_error;
use std::io::{stdin, stdout, Error, Write};

pub fn check_flags(flags: &Vec<String>, expected_flags: Vec<&str>) -> Result<(), Error> {
    let invalid_flag = flags.into_iter().find(|flag| {
        !expected_flags.contains(&flag.as_str())
            && !Command::str_is_help_flag(flag.as_str())
            && !Command::str_is_version_flag(flag)
    });

    if let Some(invalid_flag) = invalid_flag {
        return Err(invalid_input_error(&format!(
            "Invalid flag \"{}\"",
            invalid_flag
        )));
    }

    Ok(())
}

pub fn get(text: &str) -> Result<String, Error> {
    print!("{}", text);
    stdout().flush()?;

    let mut info = String::new();
    stdin().read_line(&mut info)?;

    Ok(info.trim().to_string())
}

pub fn get_valid_input<F: Fn(&str) -> bool>(
    text: &str,
    invalid_input_msg: Option<&str>,
    check_input: F,
) -> Result<String, Error> {
    let input = get(text)?;

    if check_input(&input) {
        return Ok(input);
    }

    if let Some(msg) = invalid_input_msg {
        println!("{}", msg);
    }

    get_valid_input(text, invalid_input_msg, check_input)
}

pub fn get_boolean_input(text: &str) -> Result<bool, Error> {
    let bool_value = get_valid_input(text, None, |input| {
        input == "n" || input == "y" || input == "N" || input == "Y"
    })?;

    Ok(bool_value == "y" || bool_value == "Y")
}
