use crate::cli::input::args::Args;
use crate::{
    cli::output::messages::error::{INVALID_DIRECTORY_PATH_NAME, INVALID_TEMPLATE_NAME},
    core::{repo, template::maker::make_template},
    utils::errors::{already_exists_error, invalid_input_error},
};
use std::{io::Error, path::Path, time::Instant};

pub fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    if args.inputs.len() < 1 {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_NAME));
    }
    if args.inputs.len() < 2 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let directory = args.inputs[0].clone();
    let template_name = args.inputs[1].clone();

    if repo::template_exists(&template_name) {
        return Err(already_exists_error(&format!(
            r#"Template "{}" already exists in your repository."#,
            &template_name
        )));
    }

    if !Path::new(&directory).exists() {
        return Err(invalid_input_error(&format!(
            r#"Not is possible find "{}" directory."#,
            directory
        )));
    }

    let start = Instant::now(); // start timing process
    let template = make_template(template_name, &directory)?;

    repo::save_template(template)?;
    println!("Template was saved successfully.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
