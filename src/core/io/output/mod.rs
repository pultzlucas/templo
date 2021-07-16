pub mod messages;
pub mod paint;

pub struct ProtternOutput;
use crate::{
    core::template::TemplateBundler,
    paint
};

impl ProtternOutput {
    pub fn print_template_paths(paths: Vec<&str>) {
        let paths = TemplateBundler::split_template_paths(paths);
        for (_path_type, path_name) in paths.into_iter() {
            paint!("   {gray} ", "|");
            println!("{}", path_name);
        }
    }
}
