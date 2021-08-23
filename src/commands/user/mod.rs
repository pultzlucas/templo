mod login;
mod logout;
mod signup;

use crate::cli::input::args::{parse_args, Args};
use crate::utils::errors::{invalid_input_error, not_found_error};
use crate::{
    cli::output::messages::error::NOT_FOUND_USER_AUTH,
    core::user_account::{get_user_account_data, user_auth_exists},
};
use std::io::Error;

pub async fn run(args: Args) -> Result<(), Error> {
    let sub_command = format!(
        "{} {}",
        args.command.unwrap(),
        args.inputs.join(" ")
    ).trim().to_string();
    
    let args = parse_args(sub_command)?;    

    if let Some(command) = args.command {
        return match command.as_str() {
            "login" => login::run().await,
            "signup" => signup::run().await,
            "logout" => logout::run(),
            _ => Err(invalid_input_error(&format!(
                "Invalid user subcommand \"{}\"",
                command
            ))),
        };
    }

    show_profile()
}

fn show_profile() -> Result<(), Error> {
    if !user_auth_exists() {
        return Err(not_found_error(NOT_FOUND_USER_AUTH));
    }

    let current_user = get_user_account_data()?;
    println!("{}", current_user.username);

    Ok(())
}
