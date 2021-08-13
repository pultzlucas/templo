use super::{get_user_account_data, UserAccountKey};
use crate::core::{repository::local, template::TemplateType};

pub struct UserPermissions {
    user: UserAccountKey,
}

impl UserPermissions {
    pub fn new() -> Self {
        Self {
            user: get_user_account_data().unwrap(),
        }
    }

    pub fn publish_template(&self, template_name: &str) -> bool {
        let template = local::get_template(template_name).unwrap();
        template.owner == self.user.username
    }

    pub fn delete_template(&self, template_name: &str) -> bool {
        let template = local::get_template(template_name).unwrap();
        let template_is_remote = template.template_type == TemplateType::Remote;
        template.owner == self.user.username || template_is_remote
    }
}
