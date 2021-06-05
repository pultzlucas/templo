extern crate serde_json;
use crate::core::repository::get_templates_as_struct;

pub fn templates<'a>() -> Result<&'a str, String> {
    let templates = match get_templates_as_struct() {
        Some(t) => t,
        None => return Err("Repository is empty.".to_string())
    };

    for temp in templates.iter() {
        println!("- {}", temp.name);
    }

    Ok("")
}
