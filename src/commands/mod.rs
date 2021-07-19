pub struct CommandInfo {
    name: &'static str,
    description: &'static str,
}

pub const COMMANDS: [CommandInfo; 13] = [
    CommandInfo {
        name: "save",
        description: "Saves a template from directory."
    },
    CommandInfo {
        name: "create",
        description: "Creates a project from template."
    },
    CommandInfo {
        name: "delete",
        description: "Deletes a template."
    },
    CommandInfo {
        name: "templates",
        description: "Returns all templates existents on repository."
    },
    CommandInfo {
        name: "describe",
        description: "Describes a template."
    },
    CommandInfo {
        name: "profile",
        description: "Returns the current user account info."
    },
    CommandInfo {
        name: "signup",
        description: "Registers an user account."
    },
    CommandInfo {
        name: "login",
        description: "Authenticates an user account"
    },
    CommandInfo {
        name: "logout",
        description: "Logoff current user account authenticated"
    },
    CommandInfo {
        name: "pub",
        description: "Publishes a template."
    },
    CommandInfo {
        name: "unpub",
        description: "Unpublishes a template."
    },
    CommandInfo {
        name: "get",
        description: "Installes a public template"
    },
    CommandInfo {
        name: "discover",
        description: "Show all public templates."
    }
];

/* use std::io::Error;
pub trait Command {
    fn info() -> CommandInfo;
    fn exec(args: &[String]) -> Result<(), Error>;
} */

mod create;
mod delete;
mod describe;
mod discover;
mod get;
mod help;
mod login;
mod logout;
mod profile;
mod prottern;
mod publish;
mod save;
mod signup;
mod templates;
mod unpub;
mod version;

pub use create::*;
pub use delete::*;
pub use describe::*;
pub use discover::*;
pub use get::*;
pub use help::*;
pub use login::*;
pub use logout::*;
pub use profile::*;
pub use prottern::*;
pub use publish::*;
pub use save::*;
pub use signup::*;
pub use templates::*;
pub use unpub::*;
pub use version::*;
