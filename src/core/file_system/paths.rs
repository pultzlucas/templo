pub const TEMPLATES_PATH: &'static str = r"C:\Prottern\Repository\Templates";
pub const USER_ACCOUNT_AUTH_PATH: &'static str = r"C:\Prottern\auth";

use std::path::{Path, PathBuf};
pub fn get_template_path(template_name: &String) -> PathBuf {
    Path::new(TEMPLATES_PATH).join(template_name).with_extension("tmp")
}
