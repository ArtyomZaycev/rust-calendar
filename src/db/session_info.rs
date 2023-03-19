use calendar_lib::api::{roles::types::Role, auth::types::AccessLevel};

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub user_id: i32,
    pub access_level: i32,
    pub edit_rights: bool,
    pub roles: Vec<Role>,
}

impl SessionInfo {
    pub fn has_role(&self, role: Role) -> bool {
        self.roles.iter().any(|r| *r == role)
    }

    pub fn is_max_acess_level(&self) -> bool {
        self.access_level == AccessLevel::MAX_LEVEL
    }
}
