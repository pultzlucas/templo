use crate::{
    core::{
        io::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
        repository::TemplateManager,
        user_account::UserAccountManager,
    },
    init, paintln,
};
use std::io::{Error, ErrorKind};

pub async fn publish(args: &[String]) -> Result<(), Error> {
    init()?;
    if !UserAccountManager::user_auth_exists() {
        return Err(Error::new(ErrorKind::NotFound, NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let template = TemplateManager::get_template(&args[0])?;

    paintln!("{gray}", "[Publishing Template]");
    let msg = TemplateManager::publish_template(template).await?;
    println!("{}", msg);

    Ok(())
}
