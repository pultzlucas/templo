use crate::core::repository::TemplateManager;
use std::io::{Error, ErrorKind};

pub async fn publish(args: &[String]) -> Result<(), Error> {
    if args.len() < 1 {
        let err = Error::new(ErrorKind::InvalidInput, "Template name must be specified.");
        return Err(err);
    }

    let template_name = &args[0];

    // Getting template
    let template = match TemplateManager::get_template(template_name) {
        Err(e) => return Err(e),
        Ok(t) => t,
    };
    match TemplateManager::publish_template(template).await {
        Err(e) => return Err(e),
        Ok(msg) => println!("{}", msg),
    }

    Ok(())
}
