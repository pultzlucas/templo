use crate::utils::errors::std_error;
use crate::utils::string::split_by;
use regex::Regex;
use std::io::Error;

pub struct Args {
    raw: String,
    flags: Vec<String>,
    args: Vec<String>,
}

pub fn parse_args(string_command: String) -> Result<Args, Error> {
    let raw = string_command.clone();
    let flags = get_flags(&string_command)?;
    let args = get_args(string_command)?;

    Ok(Args { raw, flags, args })
}

fn get_args(string_command: String) -> Result<Vec<String>, Error> {
    let commands = split_by(string_command, " ");
    let args = commands
        .into_iter()
        .filter(|arg| {
            let regex = Regex::new(r"(-|--)\w+").unwrap();
            !regex.is_match(arg)
        })
        .collect();
    Ok(args)
}

fn get_flags(string_command: &str) -> Result<Vec<String>, Error> {
    let regex = std_error(Regex::new(r"(-|--)\w+"))?;
    let flags: Vec<String> = regex
        .captures(&string_command)
        .into_iter()
        .enumerate()
        .map(|(i, caps)| caps.get(i).unwrap().as_str().trim().to_owned())
        .collect();
    Ok(flags)
}
