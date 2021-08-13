use crate::utils::errors::invalid_input_error;
use crate::utils::path::pathbuf_to_string;
use crate::{cli::output::messages::error::INVALID_TEMPLATE_NAME, core::repository::local};
use std::io::Error;

pub fn describe(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let template = { local::get_template(&args[0])? };

    // Describe template
    template
        .paths
        .into_iter()
        .for_each(|path| println!("{}", pathbuf_to_string(path.buf)));
    Ok(())
}
