pub mod args;

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
