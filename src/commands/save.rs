use crate::{
    cli::output::messages::error::{
        INVALID_DIRECTORY_PATH_NAME, INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH,
    },
    core::{
        repository::local,
        template::maker::create_template,
        user_account::user_auth_exists,
        utils::errors::{already_exists_error, invalid_input_error, not_found_error},
    },
};

use std::{io::Error, path::Path, time::Instant};

pub fn save(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if !user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }
    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_DIRECTORY_PATH_NAME));
    }
    if args.len() < 2 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let directory = args[0].clone();
    let template_name = args[1].clone();

    if local::template_exists(&template_name) {
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
    let template = create_template(template_name, directory)?;

    local::save_template(template)?;
    println!("Template was saved successfully.");

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
