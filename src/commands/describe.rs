use crate::{
    core::{
        io::{messages::error::INVALID_TEMPLATE_NAME},
        repository::RepositoryConnection,
        template::TemplateManager
    },
    init
};
use std::io::{Error, ErrorKind};

pub fn describe(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let repository = RepositoryConnection::new();

    let template = repository.get_template(&args[0])?;
    TemplateManager::describe_template(&template);

    Ok(())
}
