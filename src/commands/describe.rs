use crate::core::utils::errors::invalid_input_error;
use crate::core::utils::path::pathbuf_to_string;
use crate::{
    cli::output::messages::error::INVALID_TEMPLATE_NAME,
    core::repository::{create_repository_if_not_exists, RepositoryConnection},
};
use std::io::Error;

pub fn describe(args: &[String]) -> Result<(), Error> {
    create_repository_if_not_exists()?;

    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let template = {
        let repository = RepositoryConnection::new();
        repository.get_template(&args[0])?
    };

    // Describe template
    template
        .paths
        .into_iter()
        .for_each(|path| println!("{}", pathbuf_to_string(path)));
    Ok(())
}
