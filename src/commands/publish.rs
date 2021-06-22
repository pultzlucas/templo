use crate::{
    core::{
        repository::TemplateManager,
        user_account::UserAccountManager
    },
    init
};
use std::io::{Error, ErrorKind};

pub async fn publish(args: &[String]) -> Result<(), Error> {
    init()?;
    if !UserAccountManager::user_auth_exists() {
        let err = Error::new(
            ErrorKind::NotFound, 
            r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern register" to register one.
If you already have a user account created, type "prottern login" to authenticate it."#
        );
        return Err(err);
    }

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
