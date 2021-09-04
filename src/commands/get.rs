use crate::cli::input::args::Args;
use crate::core::requester::{build_request, request, validate_url, Method};
use crate::core::template::Template;
use crate::{
    cli::output::messages::error::TEMPLATE_ALREADY_EXISTS,
    core::repo,
    paintln,
    utils::errors::{already_exists_error, invalid_data_error, invalid_input_error, std_error},
};
use std::{io::Error, str, time::Instant};

pub async fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    if args.inputs.len() < 1 {
        return Err(invalid_input_error("The template url must be specified."));
    }

    let start = Instant::now(); // start timing process

    let url = validate_url(&args.inputs[0])?;

    paintln!("{gray}", "[getting template]");
    let req = build_request(url, Method::GET, None);
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
