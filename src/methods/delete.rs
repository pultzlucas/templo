use crate::cli::input::command::Command;
use crate::cli::output::messages::error::INVALID_TEMPLATE_NAME;
use crate::core::repos::Repository;
use crate::utils::errors::invalid_input_error;
use std::{io::Error, time::Instant};

pub fn run(command: Command) -> Result<(), Error> {
    if command.args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    if command.args.len() < 2 {
        return Err(invalid_input_error("Repo name must be specified."));
    }

    let templates_name = &command.args[0..command.args.len() - 2];
    let repo_name = &command.args[command.args.len() - 1];

    // Deleting templates
    let start = Instant::now(); // start timing process
    let repo = Repository::connect(repo_name.to_owned())?;

    for name in templates_name.into_iter() {
        repo.delete_template(name.to_string())?;
        println!("Template \"{}\" was deleted.", name);
    }

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
