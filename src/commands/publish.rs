use crate::{
    core::{
        io::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
        template::TemplateManager,
        repository::RepositoryConnection,
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

    let repository = RepositoryConnection::new();
    let template = repository.get_template(&args[0])?;
    let manager = TemplateManager::new(template);

    paintln!("{gray}", "[Publishing Template]");

    let msg = manager.publish_template().await?;

    println!("{}", msg);

    Ok(())
}
