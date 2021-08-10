use crate::{
    cli::output::messages::error::{INVALID_TEMPLATE_NAME, TEMPLATE_ALREADY_EXISTS},
    core::{
        repository::local,
        requester::{build_request, request, Method},
        template::Template,
        utils::errors::std_error,
    },
    paintln,
};

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::{
    io::{Error, ErrorKind},
    str,
    time::Instant,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct GetRequestBody {
    templates_name: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct GetResponseBody {
    message: String,
    getted: bool,
    templates: Vec<Template>,
}

pub async fn get(args: &[String]) -> Result<(), Error> {
    local::create()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args[0..];

    // Verify if some template already exists in repository
    for name in templates_name.iter() {
        if local::template_exists(name) {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                TEMPLATE_ALREADY_EXISTS,
            ));
        }
    }

    // Getting templates
    let start = Instant::now(); // start timing process
    let response: GetResponseBody = {
        let response = {
            let body: GetRequestBody = GetRequestBody {
                templates_name: templates_name.to_vec(),
            };
            let body_as_string: String = std_error(serde_json::to_string(&body))?;
            let req = build_request("/templates/get", Method::GET, body_as_string);
            paintln!("{gray}", "[Getting Templates]");
            request(req).await?
        };
        std_error(serde_json::from_str(&response))?
    };

    // Save templates in repository
    for temp in response.templates.into_iter() {
        local::save_template(temp.clone())?;
        println!("Template {} was installed.", temp.name);
    }

    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
