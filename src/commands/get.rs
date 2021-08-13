use crate::{
    cli::output::messages::error::{INVALID_TEMPLATE_NAME, TEMPLATE_ALREADY_EXISTS},
    core::{
        repository::{local, remote},
    },
    utils::errors::{already_exists_error, invalid_input_error},
    paintln,
};
use std::{io::Error, str, time::Instant};

pub async fn get(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args[0..];

    // Verify if some template already exists in repository
    for name in templates_name.iter() {
        if local::template_exists(name) {
            return Err(already_exists_error(TEMPLATE_ALREADY_EXISTS));
        }
    }

    paintln!("{gray}", "[Getting Templates]");

    let start = Instant::now(); // start timing process
    let templates = remote::get_templates(templates_name.to_vec()).await?;

    // Save templates in repository
    for temp in templates.into_iter() {
        local::save_template(temp.clone())?;
        println!("Template {} was installed.", temp.name);
    }

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
