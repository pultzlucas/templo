use crate::cli::input::command::Command;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use crate::{cli::output::messages::error::INVALID_TEMPLATE_NAME, core::repos};
use std::io::Error;

pub fn run(command: Command) -> Result<(), Error> {
    if command.args.len() == 0 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let template = repos::get_template(&command.args[0])?;

    // Describe template
    template
        .paths
        .into_iter()
        .for_each(|path| println!("{}", pathbuf_to_string(path.path)));

    Ok(())
}
