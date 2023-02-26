use calendar_lib::api::roles::types::Role;

pub struct SessionInfo {
    pub user_id: i32,
    pub access_level: i32,
    pub edit_rights: bool,
    pub full_access: bool, // E.g. ability to add new passwords
    pub roles: Vec<Role>,
}

impl SessionInfo {
    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}
