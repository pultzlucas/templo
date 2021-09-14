use crate::cli::input::command::Command;
use crate::{
    cli::output::messages::error::INVALID_TEMPLATE_NAME,
    core::{repo, template::maker::make_template},
    utils::errors::{already_exists_error, invalid_input_error},
};
use std::{io::Error, time::Instant};

pub fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    if command.args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let template_name = command.args[0].clone();

    if repo::template_exists(&template_name) {
        return Err(already_exists_error(&format!(
            r#"Template "{}" already exists in your repository."#,
            template_name
        )));
    }

    let start = Instant::now(); // start timing process
    let template = make_template(template_name)?;

    repo::save_template(template)?;
    println!("Template was saved successfully.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
