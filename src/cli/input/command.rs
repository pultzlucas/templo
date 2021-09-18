use crate::utils::errors::std_error;
use regex::Regex;
use std::io::Error;

#[derive(Debug, PartialEq)]
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

    pub fn get_opt_by_name(&self, name: &str) -> Option<&CommandOption> {
        self.options.iter().find(|opt| opt.name == name)
    }
}

pub fn parse_command(command: String) -> Result<Command, Error> {
    let command = command.split(" ").collect::<Vec<&str>>()[1..].join(" ");
    let flags = get_flags(&command)?;
    let options = get_options(&command)?;
    let args = get_args(&command)?;
    let method = get_method(&command)?;
    let submethod = get_submethod(&command)?;

    Ok(Command {
        flags,
        args,
        options,
        method,
        submethod,
    })
}

fn get_method(command: &str) -> Result<Option<String>, Error> {
    let method_regex = std_error(Regex::new(r"^\w+"))?;

    if let Some(caps) = method_regex.captures(command) {
        let method = caps[0].to_string();
        return Ok(Some(method));
    }

    Ok(None)
}

fn get_submethod(command: &str) -> Result<Option<String>, Error> {
    let submethod_regex_start = std_error(Regex::new(r"^\w+\s+"))?;
    let submethod_regex_end = std_error(Regex::new(r"^\w+"))?;
    let submethod_end = submethod_regex_start.replace_all(command, "");

    if let Some(caps) = submethod_regex_end.captures(submethod_end.trim()) {
        let submethod = caps[0].trim().to_string();
        return Ok(Some(submethod));
    }

    Ok(None)
}

fn get_args(command: &str) -> Result<Vec<String>, Error> {
    let args_regex = std_error(Regex::new(r"(\s--[\w-]+=[^\s]*)|(\s-[\w-]+)|(^\w+)"))?;
    let args_string = args_regex.replace_all(command, "").trim().to_string();

    let args = args_string
        .split(" ")
        .map(|arg| arg.to_string())
        .filter(|arg| !arg.is_empty());

    Ok(args.collect())
}

fn get_options(command: &str) -> Result<Vec<CommandOption>, Error> {
    let get_options = std_error(Regex::new(r"(--[\w-]+)=[^\s]*"))?;
    let get_option_name = std_error(Regex::new(r"--|=.*"))?;
    let get_option_value = std_error(Regex::new(r"--[\w-]+="))?;

    let command_options = get_options.captures_iter(command).map(|caps| {
        let option = &caps[0];
        let option_name = get_option_name.replace_all(option, "").to_string();
        let option_value = get_option_value.replace_all(option, "").to_string();

        CommandOption {
            name: option_name,
            value: option_value,
        }
    });

    Ok(command_options.collect())
}

fn get_flags(command: &str) -> Result<Vec<String>, Error> {
    let flags_regex = std_error(Regex::new(r"\s(-|--)(\w|-|[^\s])+"))?;
    let filter_regex = Regex::new(r"=").unwrap();
    let flags = flags_regex
        .captures_iter(command)
        .filter(|caps| !filter_regex.is_match(&caps[0]))
        .map(|flag| flag[0].trim().to_string());

    Ok(flags.collect())
}

#[test]
fn it_should_return_a_valid_command_struct() {
    let command = "tp method  submethod arg1 -f --flag1 ./arg2  --flag-2  arg-3 --option1=value1 arg_4 --option-2=value-2 --empty-option=".to_string();
    let struct_tested = parse_command(command).unwrap();

    let correct_struct = Command {
        method: Some("method".to_string()),
        submethod: Some("submethod".to_string()),
        flags: vec![
            "-f".to_string(),
            "--flag1".to_string(),
            "--flag-2".to_string(),
        ],
        options: vec![
            CommandOption {
                name: "option1".to_string(),
                value: "value1".to_string(),
            },
            CommandOption {
                name: "option-2".to_string(),
                value: "value-2".to_string(),
            },
            CommandOption {
                name: "empty-option".to_string(),
                value: "".to_string(),
            },
        ],
        args: vec![
            "submethod".to_string(),
            "arg1".to_string(),
            "./arg2".to_string(),
            "arg-3".to_string(),
            "arg_4".to_string(),
        ],
    };

    assert_eq!(correct_struct, struct_tested)
}
