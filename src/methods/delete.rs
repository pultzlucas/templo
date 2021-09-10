use crate::cli::input::args::Args;
use crate::utils::errors::invalid_input_error;
use crate::{cli::output::messages::error::INVALID_TEMPLATE_NAME, core::repo};
use std::{io::Error, time::Instant};

/* struct DeleteMethod {
    template_name: String,
    flags: Option<Vec<String>>,
    options: Option<Vec<String>>,
} */

pub fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    if args.inputs.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args.inputs[0..];

    // Deleting templates
    let start = Instant::now(); // start timing process

    for name in templates_name.into_iter() {
        repo::delete_template(name.to_string())?;
        println!("Template \"{}\" was deleted.", name);
    }

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
