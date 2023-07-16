use crate::api::jwt::CustomClaims;
use calendar_lib::api::{auth::types::AccessLevel, roles::types::Role};
use jwt_simple::prelude::JWTClaims;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub jwt: JWTClaims<CustomClaims>,
    pub roles: Vec<Role>,
}

impl SessionInfo {
    pub fn is_admin(&self) -> bool {
        self.has_role(Role::Admin) || self.has_role(Role::SuperAdmin)
    }

    pub fn has_role(&self, role: Role) -> bool {
        self.roles.iter().any(|r| *r == role)
    }

    pub fn get_user_id(&self) -> i32 {
        self.jwt.custom.user_id
    }
    pub fn get_access_level(&self) -> i32 {
        self.jwt.custom.access_level
    }
    pub fn get_edit_rights(&self) -> bool {
        self.jwt.custom.edit_rights
    }

    pub fn is_max_acess_level(&self) -> bool {
        self.get_access_level() == AccessLevel::MAX_LEVEL
    }
}
