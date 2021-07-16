use crate::core::file_system::paths::TEMPLATES_PATH;
use std::path::Path;

pub fn template_repository_exists() -> bool {
    Path::new(TEMPLATES_PATH).exists()
}

#[cfg(test)]
mod tests;