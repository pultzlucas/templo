use crate::{
    core::{
        io::{messages::error::INVALID_TEMPLATE_NAME, ProtternOutput},
        repository::TemplateManager,
    },
    init, paintln, paint
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

    paint!("{gray}\n", "____________________________");
    paintln!("{gray} name {gray}", "--------", "--------------");
    println!("{}", template.name);
    paint!("{gray}\n", "____________________________");

    paintln!("{gray} type {gray}", "--------", "--------------");
    println!("{:?}", template.template_type);
    paint!("{gray}\n", "____________________________");

    paintln!("{gray} owner {gray}", "--------", "-------------");
    println!("{}", template.owner);
    paint!("{gray}\n", "____________________________");

    paintln!("{gray} created at {gray}", "--------", "--------");
    println!("{}", template.created_at);
    paint!("{gray}\n", "____________________________");

    paintln!("{gray} paths {gray}", "--------", "-------------");
    let template_paths: Vec<&str> = template.paths.split(";").collect();
    ProtternOutput::print_template_paths(template_paths);

    Ok(())
}
