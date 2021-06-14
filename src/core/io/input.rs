use std::io::{stdin, stdout, Error, Write};

pub struct ProtternInput;

impl ProtternInput {
    pub fn get(text: &str) -> Result<String, Error> {
        print!("{}", text);
        stdout().flush().unwrap();
        let mut info = String::new();
        if let Err(e) = stdin().read_line(&mut info) {
            return Err(e);
        }
        Ok(info.trim().to_string())
    }
}
