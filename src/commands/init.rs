use crate::utils;
use std::{fs, path::Path};
use utils::paths::TEMPLATES_PATH;

pub fn init<'a>() -> Result<&'a str, String> {
    let repository_already_exists = !Path::new(TEMPLATES_PATH).exists();

    if repository_already_exists {
        let res = fs::create_dir_all(TEMPLATES_PATH);
    
        if res.is_err() {
            return Err(format!("Error: {}", res.unwrap_err()));
        }
            
        return Ok("Repository was created.");
    }

    Ok("")
}
