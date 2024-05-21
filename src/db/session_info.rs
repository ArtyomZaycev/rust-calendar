use std::collections::HashMap;

use crate::api::jwt::CustomClaims;
use calendar_lib::api::{permissions::types::Permissions, roles::types::Role, utils::TableId};
use jwt_simple::prelude::JWTClaims;

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub jwt: JWTClaims<CustomClaims>,
    pub user_id: TableId,
    pub shared_access: HashMap<TableId, Permissions>,
    pub roles: Vec<Role>,
}

impl SessionInfo {
    pub fn new(
        jwt: JWTClaims<CustomClaims>,
        roles: Vec<Role>,
        shared_access: HashMap<TableId, Permissions>,
    ) -> Self {
        Self {
            user_id: jwt.custom.user_id,
            jwt,
            shared_access,
            roles,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.has_role(Role::Admin) || self.has_role(Role::SuperAdmin)
    }

    pub fn has_role(&self, role: Role) -> bool {
        self.roles.iter().any(|r| *r == role)
    }

    pub fn get_permissions(&self, user_id: TableId) -> Permissions {
        if self.user_id == user_id || self.is_admin() {
            Permissions::FULL
        } else {
            self.shared_access
                .get(&user_id)
                .cloned()
                .unwrap_or(Permissions::NONE)
        }
    }
}
