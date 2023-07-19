use calendar_lib::api::roles::types::Role;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbRole {
    pub id: i32,
    pub name: String,
}
/*
impl From<Role> for DbRole {
    fn from(value: Role) -> Self {
        match value {
            Role::SuperAdmin => DbRole {
                id: value as i32,
                name: "SuperAdmin".into(),
            },
            Role::Admin => DbRole {
                id: value as i32,
                name: "Admin".into(),
            },
        }
    }
} */

impl DbRole {
    pub fn try_to_api(self) -> Option<Role> {
        match self.id {
            1 => Some(Role::SuperAdmin),
            2 => Some(Role::Admin),
            _ => None,
        }
    }
}
