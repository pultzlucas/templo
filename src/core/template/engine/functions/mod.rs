use crate::core::utils::{errors::invalid_input_error, string::split_by};
use std::io::Error;

pub trait OneParamFunction {
    fn call(args: &Vec<String>) -> String;
    fn validate_args(args: &Vec<String>) -> Result<(), Error> {
        if args.len() == 0 {
            return Err(invalid_input_error("One parameter is required."));
        }
        Ok(())
    }
}

pub trait TwoParamFunction {
    fn call(args: &Vec<String>) -> String;
    fn validate_args(args: &Vec<String>) -> Result<(), Error> {
        if args.len() < 2 {
            return Err(invalid_input_error("Two parameters is required."));
        }
        Ok(())
    }
}

pub struct Upper;
impl OneParamFunction for Upper {
    fn call(args: &Vec<String>) -> String {
        args[0].to_uppercase()
    }
}

pub struct Lower;
impl OneParamFunction for Lower {
    fn call(args: &Vec<String>) -> String {
        args[0].to_lowercase()
    }
}

pub struct UpperFirst;
impl OneParamFunction for UpperFirst {
    fn call(args: &Vec<String>) -> String {
        let first_char: Vec<String> = args[0]
            .chars()
            .enumerate()
            .into_iter()
            .map(|(i, ch)| {
                if i == 0 {
                    return ch.to_uppercase().to_string();
                }
                ch.to_string()
            })
            .collect();
        first_char.join("")
    }
}

pub struct Join;
impl TwoParamFunction for Join {
    fn call(args: &Vec<String>) -> String {
        let sep = if args[1].is_empty() { " " } else { &args[1] };
        let array = split_by(&args[0], sep);
        array.join("")
    }
}