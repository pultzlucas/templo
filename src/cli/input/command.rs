use regex::Regex;
use std::io::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct CommandOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn has_help_flag(&self) -> bool {
        self.flags
            .iter()
            .any(|flag| Self::str_is_help_flag(flag))
    }

    pub fn has_option(&self, option: &str) -> bool {
        let regex = Regex::new(&format!("{}$", option)).unwrap();
        self.options.iter().any(|opt| regex.is_match(&opt.name))
    }

    pub fn get_opt_by_name(&self, name: &str) -> Option<&CommandOption> {
        self.options.iter().find(|opt| opt.name == name)
    }

    pub fn str_is_help_flag(flag: &str) -> bool {
        flag == "--help" || flag == "-h" || flag == "-H"
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
    if command.is_empty() {
        return None;
    }

    let method_regex = Regex::new("-|=").unwrap();
    let is_method = !method_regex.is_match(&command[0]);

    if is_method {
        return Some(command[0].clone());
    }

    None
}

fn get_submethod(command: Vec<String>) -> Option<String> {
    if command.len() < 2 {
        return None;
    }
    Some(command[1].clone())
}

fn get_args(args: Vec<String>) -> Result<Vec<String>, Error> {
    if args.is_empty() {
        return Ok(vec![]);
    }

    let args = args[1..].to_vec().into_iter().filter(|arg| {
        !arg.starts_with("-") && !arg.starts_with("--") && !Regex::new("=").unwrap().is_match(arg)
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
    let flags = args.into_iter().filter(|arg| {
        (arg.starts_with("-") || arg.starts_with("--")) && !Regex::new("=").unwrap().is_match(arg)
    });

    Ok(flags.collect())
}

#[test]
fn it_should_return_a_valid_command_struct() {
    let command: Vec<String> = vec![
        "tp".to_string(),
        "method".to_string(),
        "submethod".to_string(),
        "arg1".to_string(),
        "-f".to_string(),
        "--flag1".to_string(),
        "./arg2".to_string(),
        "--flag-2".to_string(),
        "arg-3".to_string(),
        "--option1=value1".to_string(),
        "arg_4".to_string(),
        "--option-2=value-2".to_string(),
        "--empty-option=".to_string(),
        "--option_quotes=A text inside of double quotes.".to_string(),
    ];

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
            CommandOption {
                name: "option_quotes".to_string(),
                value: "A text inside of double quotes.".to_string(),
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
