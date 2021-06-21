use crate::{core::file_system::paths::TEMPLATES_PATH, paint};
use std::path::Path;
pub fn prottern() {
    paint!("{red} to {yellow}!","Welcome", "prottern");

    if !Path::new(TEMPLATES_PATH).exists() {
        println!("Type \"prottern init\" to create a template repository.")
    }
}
