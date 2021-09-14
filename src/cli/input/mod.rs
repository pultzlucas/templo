pub mod command;

use std::io::{stdin, stdout, Error, Write};

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
