use crate::core::file_system::paths::TEMPLATES_PATH;
use std::path::Path;
pub fn prottern() {
    println!("Welcome to prottern!");
    if !Path::new(TEMPLATES_PATH).exists() {
        println!("Type \"prottern init\" to create a template repository\"")
    }
}