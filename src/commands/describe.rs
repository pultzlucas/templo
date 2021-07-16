use crate::{
    core::{
        io::{messages::error::INVALID_TEMPLATE_NAME, ProtternOutput},
        template::TemplateManager,
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

    paintln!("{yellow} name", ">>");
    paint!("   {gray} ", "|");
    println!("{}\n", template.name);

    paintln!("{yellow} type", ">>");
    paint!("   {gray} ", "|");
    println!("{:?}\n", template.template_type);

    paintln!("{yellow} owner", ">>");
    paint!("   {gray} ", "|");
    println!("{}\n", template.owner);

    paintln!("{yellow} created at", ">>");
    paint!("   {gray} ", "|");
    println!("{}\n", template.created_at);

    paintln!("{yellow} paths", ">>");
    let template_paths: Vec<&str> = template.paths.split(";").collect();
    ProtternOutput::print_template_paths(template_paths);

    Ok(())
}
