use crate::{
    core::{
        io::{messages::error::INVALID_TEMPLATE_NAME, ProtternOutput},
        repository::TemplateManager,
    },
    init, paint,
};
use std::io::{Error, ErrorKind};

pub fn describe(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let template = match TemplateManager::get_template(&args[0]) {
        Ok(t) => t,
        Err(e) => return Err(Error::new(ErrorKind::NotFound, e)),
    };

    paint!("--- {yellow} ---", "name");
    println!("{}", template.name);
    paint!("--- {yellow} ---", "type");
    println!("{:?}", template.template_type);
    paint!("--- {yellow} ---", "owner");
    println!("{}", template.owner);
    paint!("--- {yellow} ---", "created at");
    println!("{}", template.created_at);
    paint!("--- {yellow} ---", "paths");

    let template_paths: Vec<&str> = template.paths.split(";").collect();
    ProtternOutput::print_template_paths(template_paths);

    Ok(())
}
