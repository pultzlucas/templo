use crate::{core::repository::template_repository_exists, paint};

pub fn prottern() {
    paint!("{red} to {yellow}!", "Welcome", "prottern");

    if !template_repository_exists() {
        paint!(
            "Type \"{yellow}\" to create a template repository.",
            "prottern init"
        );
    }
}
