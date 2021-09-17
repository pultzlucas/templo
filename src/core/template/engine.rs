use super::ConfigArg;
use crate::cli::input;
use crate::utils::errors::{not_found_error, std_error};
use regex::Regex;
use std::io::Error;

#[derive(Clone, Debug)]
pub struct TempEngineArg {
    pub key: String,
    pub value: String,
}

// Args shape: {> arg <}
pub fn parse(text: String, args: Vec<TempEngineArg>) -> Result<String, Error> {
    let shapes_regex = std_error(Regex::new(r"\{>[\w|\s|]*<}"))?;
    let key_regex = std_error(Regex::new(r"[{}><\s]"))?;

    let mut final_text = text.clone();

    for caps in shapes_regex.captures_iter(&text) {
        let shape = &caps[0];
        let key = key_regex.replace_all(shape, "");
        let arg = args.clone().into_iter().find(|arg| arg.key == key);

        if let Some(arg) = arg {
            let shape_regex = std_error(Regex::new(&format!(r"\{}", shape)))?;
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
                value: if value.is_empty() {
                    if let Some(default) = &arg.default {
                        default.to_owned()
                    } else {
                        "".to_string()
                    }
                } else {
                    value
                },
            })
        })
        .collect()
}
