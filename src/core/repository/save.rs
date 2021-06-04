use crate::core::path::get_template_dir_path;
use std::{fs, io::Error};

pub fn save_template(head: String, template_name: String) -> Result<(), Error> {
    let template_path = get_template_dir_path(&template_name);
    match fs::create_dir(&template_path) {
        Ok(o) => o,
        Err(e) => return Err(e),
    }

    let template_path = template_path
        .join("HEAD.json")
        .into_os_string()
        .into_string()
        .unwrap();
        
    fs::write(template_path, head)
}