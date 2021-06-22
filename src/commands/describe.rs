use crate::core::{io::ProtternOutput, repository::TemplateManager};
use crate::paint;
use std::io::{Error, ErrorKind};

pub fn describe(args: &[String]) -> Result<(), Error> {
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];
    let template = match TemplateManager::get_template(template_name) {
        Ok(t) => t,
        Err(e) => {
            let err = Error::new(ErrorKind::NotFound, e);
            return Err(err);
        }
    };
    paint!("--- {yellow} ---", "name");
    println!("{}", template.name);

    paint!("--- {yellow} ---", "owner");
    println!("{}", template.owner);
    
    paint!("--- {yellow} ---", "created at");
    println!("{}", template.created_at);
    
    paint!("--- {yellow} ---", "paths");
    let template_paths: Vec<&str> = template.paths.split(";").collect();
    ProtternOutput::print_template_paths(template_paths);

    Ok(())
}
