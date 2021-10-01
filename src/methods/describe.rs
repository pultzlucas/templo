use crate::cli::input::command::Command;
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::repos::Repository;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use std::io::Error;

pub fn run(command: Command) -> Result<(), Error> {
    if command.args.len() == 0 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let repo = Repository::connect(command.args[1].clone())?;
    let template = repo.get_template(&command.args[0])?;

    // Describe template
    template
        .paths
        .into_iter()
        .for_each(|path| println!("{}", pathbuf_to_string(path.path)));

    Ok(())
}
