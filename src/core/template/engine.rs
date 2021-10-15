use super::ConfigArg;
use crate::cli::input;
use crate::core::utils::errors::not_found_error;
use regex::Regex;
use std::io::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct TempEngineArg {
    pub key: String,
    pub value: String,
}

struct EngineRegex {
    pub shape: Regex,
    pub key: Regex,
}

// Args shape: {> arg <}
pub fn parse_content(content: String, args: Vec<TempEngineArg>) -> Result<String, Error> {
    parse(
        content,
        args,
        EngineRegex {
            shape: Regex::new(r"\{>[\w|\s|-]*<}").unwrap(),
            key: Regex::new(r"[{}><\s]").unwrap(),
        },
    )
}

// Args shape: ([ arg ])
pub fn parse_path(path: String, args: Vec<TempEngineArg>) -> Result<String, Error> {
    parse(
        path,
        args,
        EngineRegex {
            shape: Regex::new(r"\(\[[\w|\s|-]*\]\)").unwrap(),
            key: Regex::new(r"[\(\)\[\]\s]").unwrap(),
        },
    )
}

fn parse(text: String, args: Vec<TempEngineArg>, regex: EngineRegex) -> Result<String, Error> {
    let mut final_text = text.clone();

    for caps in regex.shape.captures_iter(&text) {
        let shape = &caps[0];
        let key = regex.key.replace_all(shape, "");
        let arg = args.clone().into_iter().find(|arg| arg.key == key);

        if let Some(arg) = arg {
            // format shape to use it as a regex
            let formated_shape = format_shape_string_to_reg(shape);

            // regex for substitute the shapes in text by key value
            let shape_regex = Regex::new(&formated_shape).unwrap();
            final_text = shape_regex.replace_all(&final_text, &arg.value).to_string();
        } else {
            return Err(not_found_error(&format!(
                "Key \"{}\" was not informed.",
                key
            )));
        }
    }

    Ok(final_text)
}

pub fn get_engine_args_input(args: &Vec<ConfigArg>) -> Result<Vec<TempEngineArg>, Error> {
    args.into_iter()
        .map(|arg| {
            let value = input::get(&arg.query)?;
            Ok(TempEngineArg {
                key: arg.key.to_string(),
                value,
            })
        })
        .collect()
}

pub fn set_arg_default_value(
    arg: TempEngineArg,
    config_args: &Vec<ConfigArg>,
) -> Result<TempEngineArg, Error> {
    if arg.value.is_empty() {
        let config_arg = config_args
            .into_iter()
            .find(|config_arg| config_arg.key == arg.key);

        if let Some(config_arg) = config_arg {
            if let Some(default_value) = &config_arg.default {
                return Ok(TempEngineArg {
                    key: arg.key,
                    value: default_value.to_owned(),
                });
            }
        }
    }

    Ok(arg)
}

fn format_shape_string_to_reg(shape: &str) -> String {
    Regex::new(r"(?P<symbol>[\()\[\]\{\}])")
    .unwrap()
    .replace_all(shape, r"\$symbol")
    .to_string()
}