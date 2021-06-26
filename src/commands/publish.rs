use crate::{
    core::{repository::TemplateManager, user_account::UserAccountManager},
    init,
    paintln
};
use std::io::{Error, ErrorKind};

pub async fn publish(args: &[String]) -> Result<(), Error> {
    init()?;
    if !UserAccountManager::user_auth_exists() {
        let err = Error::new(
            ErrorKind::NotFound,
            r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern register" to register one.
If you already have a user account created, type "prottern login" to authenticate it."#,
        );
        return Err(err);
    }

    if args.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Template name must be specified.",
        ));
    }

    let template = TemplateManager::get_template(&args[0])?;

    paintln!("{gray}", "[Publishing Template]");
    match TemplateManager::publish_template(template).await {
        Err(e) => return Err(e),
        Ok(msg) => println!("{}", msg),
    }

    Ok(())
}
