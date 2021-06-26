use crate::{
    core::{
        repository::{Template, TemplateManager},
        requester::{Method, ProtternRequester},
    },
    init,
    paintln
};
use std::io::{Error, ErrorKind};

pub async fn get(args: &[String]) -> Result<(), Error> {
    init()?;

    if args.len() < 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Template name must be specified.",
        ));
    }

    let template_name = &args[0];

    if TemplateManager::template_exists(&template_name) {
        let err = Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "Template \"{}\" already exists on repository.",
                template_name
            ),
        );
        return Err(err);
    }

    let template: Template = {
        let response = {
            let req = {
                let route = format!("/templates/get/{}", template_name);
                ProtternRequester::build_request(route.as_str(), Method::GET, "".to_string())
            };

            paintln!("{gray}", "[Getting Template]");

            ProtternRequester::request(req).await?
        };
        serde_json::from_str(&response).expect("Error when parsing JSON.")
    };

    TemplateManager::save_template(template)?;

    println!("Template was installed.");

    Ok(())
}
