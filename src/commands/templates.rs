extern crate serde_json;
use crate::core::repository::TemplateManager;
use std::io::{Error,ErrorKind};

pub fn templates<'a>() -> Result<&'a str, Error> {
    let templates = match TemplateManager::get_all_templates() {
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
