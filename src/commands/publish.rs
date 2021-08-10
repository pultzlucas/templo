use crate::paintln;
use crate::{
    cli::output::messages::error::{INVALID_TEMPLATE_NAME, NOT_FOUND_USER_AUTH},
    core::{
        repository::local,
        template::{Template, TemplateManager},
        user_account::{UserAccountManager, UserPermissions},
        utils::errors::{invalid_input_error, not_found_error},
    },
};
use std::io::{Error, ErrorKind};
use std::time::Instant;

pub async fn publish(args: &[String]) -> Result<(), Error> {
    local::create()?;
    if !UserAccountManager::user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }

    if args.len() < 1 {
        return Err(invalid_input_error(INVALID_TEMPLATE_NAME));
    }

    let templates_name = &args[0..];

    // Verify if current user has permission to publish these templates
    for name in templates_name.iter() {
        let has_permission_to = UserPermissions::new();
        if !has_permission_to.publish_template(name) {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!("You do not has permission to publish \"{}\".", name),
            ));
        }
    }

    // If all right the templates will be published
    let start = Instant::now(); // start timing process

    // Get templates from repository
    let templates: Result<Vec<Template>, Error> = templates_name
        .into_iter()
        .map(|name| local::get_template(name))
        .collect();

    // Publish templates
    let manager = TemplateManager::new(templates?);
    paintln!("{gray}", "[Publishing Templates]");
    let msg = manager.publish_templates().await?;
    println!("{}", msg);
    let end = Instant::now(); // stop timing process
    println!("Done in {:.2?}", end.duration_since(start));
    Ok(())
}
