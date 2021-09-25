use crate::cli::input;
use crate::cli::input::command::Command;
use crate::{
    core::{repo, template::maker::make_template},
    utils::errors::already_exists_error,
};
use std::{io::Error, time::Instant};

pub fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    let template_name = input::get("Template name: ")?;

    if repo::template_exists(&template_name) {
        return Err(already_exists_error(&format!(
            r#"Template "{}" already exists in your repository."#,
            template_name
        )));
    }

    let description = input::get("Template description: ")?;
    let description = if description.is_empty() {
        None
    } else {
        Some(description)
    };

    let ref_path = if !command.args.is_empty() {
        command.args[0].as_str()
    } else {
        "."
    };

    let start = Instant::now(); // start timing process
    let template = make_template(template_name, description, ref_path)?;

    repo::save_template(template)?;
    println!("Template was saved successfully.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
