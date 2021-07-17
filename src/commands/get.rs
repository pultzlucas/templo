use crate::{
    core::{
        io::messages::error::{INVALID_TEMPLATE_NAME, TEMPLATE_ALREADY_EXISTS},
        repository::{create_repository_if_not_exists, RepositoryConnection},
        requester::{Method, ProtternRequester},
        template::Template,
    },
    paintln,
};
use std::io::{Error, ErrorKind};

pub async fn get(args: &[String]) -> Result<(), Error> {
    create_repository_if_not_exists()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    let template_name = &args[0];
    let repository = RepositoryConnection::new();

    if repository.template_exists(&template_name) {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            TEMPLATE_ALREADY_EXISTS,
        ));
    }

    let template: Template = {
        let response = {
            let requester = ProtternRequester::new();
            let req = {
                let route = format!("/templates/get/{}", template_name);
                requester.build_request(route.as_str(), Method::GET, "".to_string())
            };

            paintln!("{gray}", "[Getting Template]");

            requester.request(req).await?
        };
        serde_json::from_str(&response).expect("Error when parsing JSON.")
    };

    RepositoryConnection::new().save_template(template)?;

    println!("Template was installed.");

    Ok(())
}
