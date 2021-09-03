use crate::cli::input::args::Args;
use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use crate::{cli::output::messages::error::INVALID_TEMPLATE_NAME, core::repo};
use std::io::Error;

pub fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    if args.inputs.len() == 0 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let template = repo::get_template(&args.inputs[0])?;

    // Describe template
    template
        .paths
        .into_iter()
        .for_each(|path| println!("{}", pathbuf_to_string(path.path)));

    Ok(())
}
