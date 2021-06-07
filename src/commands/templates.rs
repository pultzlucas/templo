extern crate serde_json;
use crate::core::repository::get_templates_as_struct;
use std::io::{Error,ErrorKind};

pub fn templates<'a>() -> Result<&'a str, Error> {
    let templates = match get_templates_as_struct() {
        Some(t) => t,
        None => {
            let err = Error::new(ErrorKind::NotFound, "Repository is empty.");
            return Err(err);
        }
    };

    for temp in templates.iter() {
        println!("- {}", temp.name);
    }

    Ok("")
}
