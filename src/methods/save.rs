use crate::cli::input;
use crate::cli::input::command::Command;
use crate::core::repos::Repository;
use crate::{core::template::maker::make_template, utils::errors::already_exists_error};
use std::{io::Error, time::Instant};

pub fn run(command: Command) -> Result<(), Error> {
    let template_name = if command.has_option("name") {
        command.get_opt_by_name("name").unwrap().value.clone()
    } else {
        input::get("Template name: ")?
    };

    let repo = Repository::connect(command.args[1].clone())?;

    if repo.template_exists(&template_name) {
        return Err(already_exists_error(&format!(
            r#"Template "{}" already exists in your repository."#,
            template_name
        )));
    }

    let description_value = if command.has_option("description") {
        command
            .get_opt_by_name("description")
            .unwrap()
            .value
            .clone()
    } else {
        input::get("Template description: ")?
    };

    let description = if description_value.is_empty() {
        None
    } else {
        Some(description_value)
    };

    let ref_path = if !command.args.is_empty() {
        command.args[0].as_str()
    } else {
        "."
    };

    let start = Instant::now(); // start timing process
    let template = make_template(template_name, description, ref_path)?;

    repo.save_template(template)?;
    println!("Template was saved successfully.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
