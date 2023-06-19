use calendar_lib::api::roles::types::Role;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbRole {
    pub id: i32,
    pub name: String,
}

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
}

impl TryFrom<DbRole> for Role {
    type Error = ();

    fn try_from(value: DbRole) -> Result<Self, Self::Error> {
        match value.id {
            1 => Ok(Self::SuperAdmin),
            2 => Ok(Self::Admin),
            _ => Err(()),
        }
    }
}
