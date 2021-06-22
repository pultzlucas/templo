use crate::core::{
    file_system::paths::TEMPLATES_PATH,
    repository::template_repository_exists
};
use std::{fs, io::Error};

pub fn init() -> Result<(), Error> {
    if !template_repository_exists() {
        if let Err(e) = fs::create_dir_all(TEMPLATES_PATH) {
            return Err(e);
        }
        println!("Repository was created.");

        return Ok(());
    }

    Ok(())
}
