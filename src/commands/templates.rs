extern crate serde_json;
use crate::core::repository::get_templates_as_struct;

pub fn templates<'a>() -> Result<&'a str, String> {
    let templates = get_templates_as_struct();

    for temp in templates.iter() {
        println!("- {}", temp.name);
    }

    Ok("")
}
