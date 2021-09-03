use crate::cli::input::args::Args;
use crate::{
    cli::output::messages::error::{INVALID_TEMPLATE_NAME},
    core::repository::local,
};
use std::{
    io::{Error, ErrorKind},
    time::Instant,
};

pub fn run(args: Args) -> Result<(), Error> {
    local::create()?;

    if args.inputs.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args.inputs[0..];

    // Deleting templates
    let start = Instant::now(); // start timing process
    for name in templates_name.iter() {
        local::delete_template(name)?;
        println!("Template \"{}\" was deleted.", name);
    }
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));
    Ok(())
}
