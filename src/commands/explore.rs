use crate::{
    core::{
        requester::{Method, ProtternRequester},
        template::Template,
    },
    paintln,
};
use std::io::Error;
use tabled::{Disable, Style, Table};

pub async fn explore() -> Result<(), Error> {
    let templates: Vec<Template> = {
        let requester = ProtternRequester::new();
        let req = requester.build_request("/templates", Method::GET, "".to_owned());

        paintln!("{gray}", "[Searching Templates]");

        let response = requester.request(req).await?;
        serde_json::from_str(&response).unwrap()
    };

    let templates_tb = Table::new(templates)
        .with(Disable::Column(4..))
        .with(Disable::Column(3..4))
        .with(Style::pseudo());

    print!("{}", templates_tb);

    Ok(())
}
