use crate::core::file_system::paths::TEMPLATES_PATH;
use std::{fs, io::Error, path::Path};

pub fn init() -> Result<(), Error> {
    let repository_not_exists = !Path::new(TEMPLATES_PATH).exists();

    if repository_not_exists {
        if let Err(e) = fs::create_dir_all(TEMPLATES_PATH) {
            return Err(e);
        }
        println!("Repository was created.");

        return Ok(());
    }

    Ok(())
}
