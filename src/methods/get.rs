use crate::cli::input::command::Command;
use crate::core::namespaces::parse_to_raw_url;
use crate::core::repo;
use crate::core::requester::{build_request, request, str_is_url, validate_url, Method};
use crate::core::template::Template;
use crate::{
    cli::output::messages::error::TEMPLATE_ALREADY_EXISTS,
    paintln,
    utils::errors::{already_exists_error, invalid_data_error, invalid_input_error, std_error},
};
use std::{io::Error, str, time::Instant};

pub async fn run(command: Command) -> Result<(), Error> {
    repo::create()?;

    if command.args.len() < 1 {
        return Err(invalid_input_error("The template url must be specified."));
    }

    let start = Instant::now(); // start timing process

    let url = if str_is_url(&command.args[0]) {
        validate_url(&command.args[0])?.to_string()
    } else {
        let template_url_path = command.args[0].clone();
        let url = parse_to_raw_url(template_url_path)?;
        validate_url(&url)?.to_string()
    };

    paintln!("{gray}", "[getting template]");
    let req = build_request(&url, Method::GET, None);
    let res = request(req).await?;

    // check if template data is valid
    if serde_json::from_str::<Template>(&res).is_err() {
        return Err(invalid_data_error("Template data is incorrect."));
    }

    let template: Template = std_error(serde_json::from_str(&res))?;

    //check if a template with the same name already exists in repo
    if repo::template_exists(&template.name) {
        return Err(already_exists_error(TEMPLATE_ALREADY_EXISTS));
    }

    repo::save_template(template.clone())?;
    println!("Template \"{}\" was installed.", template.name);

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
