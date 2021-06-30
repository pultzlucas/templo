pub mod messages;
pub mod paint;

pub struct ProtternOutput;
use crate::core::repository::TemplateManager;

impl ProtternOutput {
    pub fn print_template_paths(paths: Vec<&str>) {
        let paths = TemplateManager::split_template_paths(paths);
        for (path_type, path_name) in paths.into_iter() {
            println!("{:>4}: {}", path_type, path_name);
        }
    }
}
