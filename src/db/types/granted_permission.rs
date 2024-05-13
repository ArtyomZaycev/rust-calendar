use calendar_lib::api::permissions::types::{GrantedPermission, Permissions};
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbGrantedPermission {
    pub id: i32,
    pub giver_user_id: i32,
    pub receiver_user_id: i32,
    pub permissions_id: i32,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::granted_permissions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewGrantedPermission {
    pub giver_user_id: i32,
    pub receiver_user_id: i32,
    pub permissions_id: i32,
}

impl DbGrantedPermission {
    pub fn to_api(self, permissions: Permissions) -> GrantedPermission {
        GrantedPermission {
            id: self.id,
            giver_user_id: self.giver_user_id,
            receiver_user_id: self.receiver_user_id,
            permissions,
        }
    }
}
