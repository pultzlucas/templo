use crate::utils::errors::std_error;
use crate::utils::string::split_by;
use regex::Regex;
use std::io::Error;

#[derive(Debug)]
pub struct Args {
    pub raw: String,
    pub program: String,
    pub command: Option<String>,
    pub flags: Vec<String>,
    pub inputs: Vec<String>,
}

impl Args {
    pub fn has_flag(&self, flag: &str) -> bool {
        let regex = Regex::new(flag).unwrap();
        self.flags.iter().any(|flag| regex.is_match(flag))
    }
}

pub fn parse_args(string_command: String) -> Result<Args, Error> {
    let args = &split_by(string_command.clone(), " ")[1..];
    let raw = string_command.clone();
    let program = split_by(string_command.clone(), " ")[0].clone();
    let command = if args.len() > 0 {
        Some(args[0].clone())
    } else {None};
    let flags = get_flags(&args.join(" "))?;
    let inputs = if args.len() > 1 {
        get_inputs(args[1..].join(" "))?
    }else {vec![]};

    Ok(Args {
        raw,
        program,
        command,
        flags,
        inputs,
    })
}

fn get_inputs(string_command: String) -> Result<Vec<String>, Error> {
    let commands = split_by(string_command, " ");
    let args: Vec<String> = commands
        .into_iter()
        .filter(|arg| {
            let regex = Regex::new(r"\s(-|--)\w+").unwrap();
            !regex.is_match(arg)
        })
        .collect();

    Ok(args)
}

fn get_flags(string_command: &str) -> Result<Vec<String>, Error> {
    let regex = std_error(Regex::new(r"\s(-|--)\w+"))?;
    let flags: Vec<String> = regex
        .captures(&string_command)
        .into_iter()
        .enumerate()
        .map(|(i, caps)| caps.get(i).unwrap().as_str().trim().to_owned())
        .collect();
    Ok(flags)
}
