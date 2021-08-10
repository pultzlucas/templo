use super::Template;
use crate::{core::utils::path::pathbuf_to_string, paint_string};
use std::{fs, io::Error, path::Path};

pub struct TemplateManager {
    templates: Vec<Template>,
}

impl TemplateManager {
    pub fn new(templates: Vec<Template>) -> Self {
        Self { templates }
    }

    pub fn gen_templates(&self, directory: &Path) -> Result<(), Error> {
        for template in self.templates.clone().into_iter() {
            // creating files and directories
            for path in template.paths.into_iter() {
                let real_path = Path::new(directory)
                    .join(pathbuf_to_string(path))
                    .to_path_buf();
                if real_path.is_file() {
                    fs::write(&real_path, "")?;
                    println!("{} {:?}", paint_string!("{gray}", "file:"), real_path);
                }

                if real_path.is_dir() {
                    fs::create_dir(&real_path)?;
                    println!(" {} {:?}", paint_string!("{gray}", "dir:"), real_path);
                }
            }

            if template.contents.len() > 0 {
                // writing the files content
                for data in template.contents.into_iter() {
                    let file_path = Path::new(directory).join(data.filename);
                    if file_path.exists() {
                        fs::write(file_path, data.content)?;
                    }
                }
            };
        }

        Ok(())
    }
}
