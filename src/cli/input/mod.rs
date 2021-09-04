pub mod args;

/* use std::io::{stdin, stdout, Error, Write};

pub fn get(text: &str) -> Result<String, Error> {
    print!("{}", text);
    stdout().flush()?;

    let mut info = String::new();
    stdin().read_line(&mut info)?;

    Ok(info.trim().to_string())
} */

/* pub fn get_valid_input<F: Fn(&str) -> bool>(
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
 */