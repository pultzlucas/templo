use std::path::Path;
use crate::utils::paths::TEMPLATES_PATH;

pub fn template_exists(template_name: &String) -> bool {
    Path::new(TEMPLATES_PATH).join(template_name).exists()
}