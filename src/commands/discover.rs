use crate::{
    core::{
        requester::{Method, ProtternRequester},
        template::Template,
    },
    paintln,
};

use std::io::Error;

pub async fn discover() -> Result<(), Error> {
    let templates: Vec<Template> = {
        let requester = ProtternRequester::new();
        let req = requester.build_request("/templates", Method::GET, "".to_owned());

        paintln!("{gray}", "[Searching Templates]");

        let response = requester.request(req).await?;
        serde_json::from_str(&response).unwrap()
    };

    for template in templates.into_iter() {
        println!("- {}", template.name);
    }

    Ok(())
}
