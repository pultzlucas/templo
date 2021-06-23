mod template_manager;
mod template;
mod template_bundler;

pub use template::*;
pub use template_manager::TemplateManager;
pub use template_bundler::TemplateFormatter;

use crate::core::file_system::paths::TEMPLATES_PATH;
use std::path::Path;

pub fn template_repository_exists() -> bool {
    Path::new(TEMPLATES_PATH).exists()
}