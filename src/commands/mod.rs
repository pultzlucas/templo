pub const COMMANDS: [(&str, &str);13] = [
    ("init", "Creates a local repository."),
    ("save", "Saves a template from directory."),
    ("create", "Creates a project from template."),
    ("delete", "Deletes a template."),
    ("templates", "Returns all templates existents on repository."),
    ("describe", "Describes a template."),
    ("profile", "Returns the current user account info."),
    ("register", "Registers an user account."),
    ("login", "Authenticates an user account"),
    ("pub", "Publishes a template."),
    ("unpub", "Unpublishes a template."),
    ("get", "Installes a public template"),
    ("discover", "Show all public templates."),
];

mod init;
mod prottern;
mod save;
mod delete;
mod templates;
mod describe;
mod create;
mod register;
mod login;
mod publish;
mod unpub;
mod get;
mod discover;
mod profile;
mod help;

pub use init::*;
pub use prottern::*;
pub use save::*;
pub use delete::*;
pub use templates::*;
pub use describe::*;
pub use create::*;
pub use register::*;
pub use login::*;
pub use publish::*;
pub use unpub::*;
pub use get::*;
pub use discover::*;
pub use profile::*;
pub use help::*;
