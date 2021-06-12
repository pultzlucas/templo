use std::{fs, path::Path, io::{Error, ErrorKind}};
use crate::core::file_system::paths::TEMPLATES_PATH;

pub fn init() -> Result<(), Error> {
    let repository_not_exists = !Path::new(TEMPLATES_PATH).exists();

    if repository_not_exists {
        let res = fs::create_dir_all(TEMPLATES_PATH);
    
        if res.is_err() {
            let err = Error::new(ErrorKind::Interrupted, res.unwrap_err());
            return Err(err);
        }
            
        println!("Repository was created.");

        return Ok(());
    }

    Ok(())
}
