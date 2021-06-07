use std::io::{Error, ErrorKind};
use crate::core::repository::get_template;

pub fn describe(args: &[String]) -> Result<&str, Error> {
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];
    let template = match get_template(template_name) {
        Ok(t) => t,
        Err(e) => {
            let err = Error::new(ErrorKind::NotFound, e);
            return Err(err);
        }
    };
    
    let paths_splitted: Vec<&str> = template.paths.split(";").collect();

    println!("--- name ---");
    println!("{}", template.name);

    println!("--- paths ---");
    paths_splitted.iter().for_each(|path| {
        println!("{}", path);
    });

    Ok("")
}
