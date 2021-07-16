use crate::core::file_system::paths::TEMPLATES_PATH;
use std::path::Path;

mod repository_connection;
pub use repository_connection::*;

pub fn repository_exists() -> bool {
    Path::new(TEMPLATES_PATH).exists()
}
