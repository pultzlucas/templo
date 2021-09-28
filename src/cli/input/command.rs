use regex::Regex;
use std::io::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct CommandOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    pub method: Option<String>,
    pub submethod: Option<String>,
    pub flags: Vec<String>,
    pub args: Vec<String>,
    pub options: Vec<CommandOption>,
}

impl Command {
    pub fn has_flag(&self, flag: &str) -> bool {
        let regex = Regex::new(&format!("{}$", flag)).unwrap();
        self.flags.iter().any(|flag| regex.is_match(flag))
    }

    pub fn has_option(&self, option: &str) -> bool {
        let regex = Regex::new(&format!("{}$", option)).unwrap();
        self.options.iter().any(|opt| regex.is_match(&opt.name))
    }

    pub fn get_opt_by_name(&self, name: &str) -> Option<&CommandOption> {
        self.options.iter().find(|opt| opt.name == name)
    }
}

pub fn parse_command(args: Vec<String>) -> Result<Command, Error> {
    let command = args[1..].to_vec();
    let flags = get_flags(command.clone())?;
    let options = get_options(command.clone())?;
    let args = get_args(command.clone())?;
    let method = get_method(command.clone());
    let submethod = get_submethod(command.clone());

    Ok(Command {
        flags,
        args,
        options,
        method,
        submethod,
    })
}

fn get_method(command: Vec<String>) -> Option<String> {
    if command.len() < 1 {
        return None;
    }
    Some(command[0].clone())
}

fn get_submethod(command: Vec<String>) -> Option<String> {
    if command.len() < 2 {
        return None;
    }
    Some(command[1].clone())
}

fn get_args(args: Vec<String>) -> Result<Vec<String>, Error> {
    let args = args
        .into_iter()
        .filter(|arg| {
            !arg.starts_with("-") 
            && !arg.starts_with("--")
            && !Regex::new("=").unwrap().is_match(arg)
        });

    Ok(args.collect())
}

fn get_options(args: Vec<String>) -> Result<Vec<CommandOption>, Error> {
    let options_regex = Regex::new(r"--[\w-]+=").unwrap();
    let options = args
        .into_iter()
        .filter(|arg| options_regex.is_match(arg))
        .map(|option| {
            let name = options_regex
                .find(&option)
                .unwrap()
                .as_str()
                .to_string()
                .replace("--", "")
                .replace("=", "");
            let value = options_regex.replace_all(&option.clone(), "").to_string();
            CommandOption { name, value }
        });

    Ok(options.collect())
}

fn get_flags(args: Vec<String>) -> Result<Vec<String>, Error> {
    let flags = args
        .into_iter()
        .filter(|arg| {
            (arg.starts_with("-") || arg.starts_with("--"))
            && !Regex::new("=").unwrap().is_match(arg)
        });

    Ok(flags.collect())
}
