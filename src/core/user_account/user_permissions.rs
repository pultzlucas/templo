use crate::core::{
    template::TemplateType,
    repository::RepositoryConnection
};
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
        let repository = RepositoryConnection::new();
        let template = repository.get_template(template_name).unwrap();
        template.metadata.owner == self.user.username
    }

    pub fn delete_template(&self, template_name: &String) -> bool {
        let repository = RepositoryConnection::new();
        let template = repository.get_template(template_name).unwrap();
        let template_is_remote = template.metadata.template_type == TemplateType::Remote; 
        template.metadata.owner == self.user.username || template_is_remote
    }
}