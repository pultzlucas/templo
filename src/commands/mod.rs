use tabled::Tabled;

#[derive(Tabled)]
pub struct CommandInfo {
    command: &'static str,
    description: &'static str,
}

pub const COMMANDS: [CommandInfo; 13] = [
    CommandInfo {
        command: "save",
        description: "Saves a template from directory."
    },
    CommandInfo {
        command: "generate/gen",
        description: "Generates a project from template."
    },
    CommandInfo {
        command: "delete/del",
        description: "Deletes a template."
    },
    CommandInfo {
        command: "repository/repo",
        description: "Returns all templates existents on repository."
    },
    CommandInfo {
        command: "describe/desc",
        description: "Describes a template."
    },
    CommandInfo {
        command: "profile",
        description: "Returns the current user account info."
    },
    CommandInfo {
        command: "signup",
        description: "Registers an user account."
    },
    CommandInfo {
        command: "login",
        description: "Authenticates an user account"
    },
    CommandInfo {
        command: "logout",
        description: "Logoff current user account authenticated"
    },
    CommandInfo {
        command: "pub",
        description: "Publishes a template."
    },
    CommandInfo {
        command: "unpub",
        description: "Unpublishes a template."
    },
    CommandInfo {
        command: "get",
        description: "Installes a public template"
    },
    CommandInfo {
        command: "explore",
        description: "Show all public templates."
    }
];

/* use std::io::Error;
pub trait Command {
    fn info() -> CommandInfo;
    fn exec(args: &[String]) -> Result<(), Error>;
} */

mod generate;
mod delete;
mod describe;
mod explore;
mod get;
mod help;
mod login;
mod logout;
mod profile;
mod prottern;
mod publish;
mod save;
mod signup;
mod repository;
mod unpub;
mod version;

pub use generate::*;
pub use delete::*;
pub use describe::*;
pub use explore::*;
pub use get::*;
pub use help::*;
pub use login::*;
pub use logout::*;
pub use profile::*;
pub use prottern::*;
pub use publish::*;
pub use save::*;
pub use signup::*;
pub use repository::*;
pub use unpub::*;
pub use version::*;
