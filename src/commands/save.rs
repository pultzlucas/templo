use crate::core::path::ProtternFileSystem;
use crate::core::repository::{TemplateManager, Template};
use std::{
    io::{Error, ErrorKind},
    path::Path,
};

pub fn save(args: &[String]) -> Result<(), Error> {
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

    if ProtternFileSystem::get_dir_address(&template_name).exists() {
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

    let (template_paths, template_content) = match ProtternFileSystem::extract_template_from(directory) {
        Ok(o) => o,
        Err(e) => {
            let err = Error::new(ErrorKind::Other, e);
            return Err(err);
        },
    };

    let template = Template::new(template_name, template_paths, template_content);
    
    if let Err(e) = TemplateManager::save_template(template) {
        return Err(e);
    }

    println!("Template was saved successfully.");

    Ok(())
}
