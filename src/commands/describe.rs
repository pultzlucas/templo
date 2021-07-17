use crate::core::{
    io::messages::error::INVALID_TEMPLATE_NAME, repository::create_repository_if_not_exists,
    repository::RepositoryConnection, template::TemplateManager,
};
use std::io::{Error, ErrorKind};

pub fn describe(args: &[String]) -> Result<(), Error> {
    create_repository_if_not_exists()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let template = {
        let repository = RepositoryConnection::new();
        repository.get_template(&args[0])?
    };
    let manager = TemplateManager::new(template);
    manager.describe_template();

    Ok(())
}
