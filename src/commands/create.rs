use crate::{core::repository::TemplateManager, init};
use std::{
    fs,
    io::{Error, ErrorKind, Write},
    path::Path,
};

pub fn create(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    if args.len() < 2 {
        let err = Error::new(ErrorKind::InvalidInput, "Directory path must be specified.");
        return Err(err);
    }
    let template_name = &args[0];
    let directory = Path::new(&args[1]);
    if directory.extension() != None {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "The path should be a directory.",
        ));
    }
    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }
    let template = match TemplateManager::get_template(template_name) {
        Ok(t) => t,
        Err(e) => return Err(Error::new(ErrorKind::NotFound, e)),
    };

    let template_paths: Vec<(&str, &Path)> = {
        let template_paths_splitted: Vec<&str> = template.paths.split(";").collect();
        template_paths_splitted
            .into_iter()
            .map(|path| {
                let path_splitted: Vec<&str> = path.split("|").collect();
                (path_splitted[0], Path::new(path_splitted[1]))
            })
            .collect()
    };
    let template_content: Vec<(&Path, Vec<u8>)> = {
        let template_content_splitted: Vec<&str> = template.content.split(";").collect();
        template_content_splitted
            .into_iter()
            .map(|content| {
                let content_splitted: Vec<&str> = content.split("|").collect();
                (
                    Path::new(content_splitted[0]),
                    base64::decode(content_splitted[1]).unwrap(),
                )
            })
            .collect()
    };

    // creating files and directories
    for (path_type, path_name) in template_paths.into_iter() {
        let real_path = Path::new(directory).join(path_name);
        if path_type == "file" {
            if let Err(e) = fs::write(&real_path, "") {
                return Err(e);
            }
        }
        if path_type == "dir" {
            if let Err(e) = fs::create_dir(&real_path) {
                return Err(e);
            }
        }
    }

    // writing the project content
    for (file_name, content_buf) in template_content.into_iter() {
        let real_file_path = Path::new(directory).join(file_name);
        if real_file_path.exists() {
            let mut file = fs::OpenOptions::new().write(true).open(real_file_path)?;
            if let Err(e) = file.write(&content_buf[..]) {
                return Err(e);
            }
        }
    }
    println!("Project was created.");
    Ok(())
}
