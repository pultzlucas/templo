use crate::core::repository::{TemplateManager, TemplateType};
use super::{UserAccountKey, UserAccountManager};

pub struct UserPermissions {
    user: UserAccountKey
}

impl UserPermissions {
    pub fn new() -> Self {
        Self {
            user: UserAccountManager::get_user_account_data().unwrap()
        }
    }

    pub fn publish_template(&self, template_name: &String) -> bool {
        let template = TemplateManager::get_template(template_name).unwrap();
        template.owner == self.user.username
    }

    pub fn delete_template(&self, template_name: &String) -> bool {
        let template = TemplateManager::get_template(template_name).unwrap();
        let template_is_remote = template.template_type == TemplateType::Remote; 
        template.owner == self.user.username || template_is_remote
    }
}