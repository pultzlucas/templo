pub mod error {
    pub const NOT_FOUND_USER_AUTH: &'static str = r#"This process cannot be runned because You dont has an authenticated user account.
Please type "prottern signup" to register one.
If you already have a user account registered, type "prottern login" to authenticate it."#;

    pub const INVALID_TEMPLATE_NAME: &'static str = "Template name must be specified.";
    pub const INVALID_DIRECTORY_PATH_NAME: &'static str = "Directory path must be specified.";
    pub const INVALID_DIRECTORY_PATH: &'static str = "Invalid directory path.";
    pub const INVALID_DIRECTORY_PATH_TYPE: &'static str = "The path should be a directory.";
    pub const TEMPLATE_ALREADY_EXISTS: &'static str = "Template already exists on repository.";
}