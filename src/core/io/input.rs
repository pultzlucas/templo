extern crate scanpw;
use scanpw::scanpw;
use std::io::{stdin, stdout, Error, Write};

pub struct ProtternInput;

pub enum InputType {
    Text,
    Password
}

impl ProtternInput {
    pub fn get(text: &str, input_type: InputType) -> Result<String, Error> {
        print!("{}", text);
        stdout().flush()?;
        
        match input_type {
            InputType::Text => {
                let mut info = String::new();
                if let Err(e) = stdin().read_line(&mut info) {
                    return Err(e);
                }
                Ok(info.trim().to_string())
            },
            
            InputType::Password => {
                let password = scanpw!();
                Ok(password.trim().to_string())
            }
        }

    }
}
