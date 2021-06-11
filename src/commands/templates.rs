extern crate serde_json;
use crate::core::repository::TemplateManager;
use std::io::Error;

pub fn templates() -> Result<(), Error> {
    if let Some(templates) = TemplateManager::get_all_templates() {
        for temp in templates.iter() {
            println!("- {}", temp.name);
        }
    } else {
        println!("Repository is empty.");
    }

    Ok(())
}
