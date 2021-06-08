use crate::core::path::ProtternFileSystem;
use crate::core::repository::TemplateManager;
use crate::utils::structs::Template;
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

pub fn save(args: &[String]) -> Result<&str, Error> {
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Folder name must be specified.");
        return Err(err);
    }
    if args.len() < 2 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let directory = args[0].clone();
    let template_name = args[1].clone();

    if ProtternFileSystem::get_dir_path(&template_name).exists() {
        let err = Error::new(
            ErrorKind::AlreadyExists,
            format!("Template \"{}\" already exists.", &template_name),
        );
        return Err(err);
    }
    
    if !Path::new(&directory).exists() {
        let err = Error::new(
            ErrorKind::InvalidInput,
            format!("Directory {} not exists.", directory),
        );
        return Err(err);
    }

    let template_paths = match ProtternFileSystem::dismount_dir(directory) {
        Ok(o) => o,
        Err(e) => {
            let err = Error::new(ErrorKind::Other, e);
            return Err(err);
        },
    };
    
    let head = Template {
        name: template_name,
        paths: template_paths.join(";"),
    };
    
    let head_string = serde_json::to_string_pretty(&head).unwrap();
    
    if let Err(e) = TemplateManager::save_template(head_string, head.name) {
        return Err(e);
    }

    Ok("Template was saved successfully.")
}
