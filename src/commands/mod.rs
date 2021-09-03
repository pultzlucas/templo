use tabled::Tabled;

#[derive(Tabled)]
pub struct CommandInfo {
    command: &'static str,
    description: &'static str,
}

pub const COMMANDS: [CommandInfo; 14] = [
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
        description: "Authenticates an user account."
    },
    CommandInfo {
        command: "logout",
        description: "Logoff current user account authenticated."
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
        description: "Installes a public template."
    },
    CommandInfo {
        command: "explore",
        description: "Show all public templates."
    },
    CommandInfo {
        command: "documentation/docs",
        description: "Open the Prottern documentation."
    }
];

/* use std::io::Error;
pub trait Command {
    fn info() -> CommandInfo;
    fn exec(args: &[String]) -> Result<(), Error>;
} */

pub mod generate;
pub mod delete;
pub mod describe;
pub mod get;
pub mod help;
pub mod prottern;
pub mod save;
pub mod repository;
pub mod version;
pub mod documentation;
