use crate::core::{io::ProtternOutput, repository::TemplateManager};
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
    println!("--- name ---");
    println!("{}", template.name);

    println!("--- owner ---");
    println!("{}", template.owner);

    println!("--- created at ---");
    println!("{}", template.created_at);

    println!("--- paths ---");
    let template_paths: Vec<&str> = template.paths.split(";").collect();
    ProtternOutput::print_template_paths(template_paths);

    Ok(())
}
