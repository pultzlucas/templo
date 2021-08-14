use crate::{
    cli::output::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
    core::{repository::remote, user_account::user_auth_exists},
    paintln,
    utils::errors::{invalid_input_error, not_found_error},
};
use serde_derive::{Deserialize, Serialize};
use std::io::Error;
use std::time::Instant;

#[derive(Deserialize, Serialize)]
struct UnpubRequestBody {
    templates_name: Vec<String>,
    user: String,
}

pub async fn run(args: &[String]) -> Result<(), Error> {
    if !user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }
    let templates_name = &args[0..];

    paintln!("{gray}", "[Unpublishing Templates]");

    let start = Instant::now(); // start timing process
    let res = remote::unpub_templates(templates_name.to_vec()).await?;
    println!("{}", res);
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));

    Ok(())
}
