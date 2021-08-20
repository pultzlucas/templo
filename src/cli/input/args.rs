use crate::utils::errors::std_error;
use crate::utils::string::split_by;
use regex::Regex;
use std::io::Error;

#[derive(Debug)]
pub struct Args {
    pub raw: String,
    pub program_name: String,
    pub command: String,
    pub flags: Vec<String>,
    pub args: Vec<String>,
}

impl Args {
    pub fn has_flag(&self, flag: &str) -> bool {
        let regex = Regex::new(flag).unwrap();
        self.flags.iter().any(|flag| regex.is_match(flag))
    }
}

pub fn parse_args(string_command: String) -> Result<Args, Error> {
    let command_split = split_by(string_command.clone(), " ")[1..]
        .to_vec()
        .join(" ");
    let raw = string_command.clone();
    let program_name = split_by(string_command.clone(), " ")[0].clone();
    let command = split_by(command_split.clone(), " ")[0].clone();
    let flags = get_flags(&command_split)?;
    let args = get_args(command_split)?;

    Ok(Args {
        raw,
        program_name,
        command, 
        flags,
        args,
    })
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
