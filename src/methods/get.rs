use crate::cli::input::args::Args;
use crate::core::config::registry::remote_repos;
use crate::core::repo;
use crate::core::requester::{build_request, request, validate_url, Method};
use crate::core::template::Template;
use crate::methods::check_flags;
use crate::{
    cli::output::messages::error::TEMPLATE_ALREADY_EXISTS,
    paintln,
    utils::errors::{
        already_exists_error, invalid_data_error, invalid_input_error, not_found_error, std_error,
    },
};
use std::{io::Error, str, time::Instant};

pub async fn run(args: Args) -> Result<(), Error> {
    repo::create()?;

    let flags = vec!["--url"];
    check_flags(&args.flags, flags)?;

    if args.inputs.len() < 1 {
        return Err(invalid_input_error("The template url must be specified."));
    }

    let start = Instant::now(); // start timing process

    let url = if args.has_flag("--url") {
        validate_url(&args.inputs[0])?.to_string()
    } else {
        if args.inputs.len() < 2 {
            return Err(invalid_input_error("Repo name must be specified"));
        }

        let repo_name = args.inputs[1].clone();
        let repo_registry = remote_repos::get_repo_registry(&repo_name)?;

        if let Some(repo) = repo_registry {
            let url = format!("{}/templates/{}", repo.url, args.inputs[0]);
            validate_url(&url)?.to_string()
        } else {
            return Err(not_found_error(&format!(
                "Repo \"{}\" not is registered.",
                repo_name
            )));
        }
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
