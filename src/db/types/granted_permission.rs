use calendar_lib::api::{
    permissions::types::{
        GrantedPermission, NewGrantedPermission, Permissions, UpdateGrantedPermission,
    },
    utils::TableId,
};
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

#[derive(diesel::AsChangeset)]
#[diesel(table_name = crate::db::schema::granted_permissions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbUpdateGrantedPermission {
    pub id: i32,
    pub giver_user_id: Option<i32>,
    pub receiver_user_id: Option<i32>,
    pub permissions_id: Option<i32>,
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

impl DbNewGrantedPermission {
    pub fn from_api(
        value: NewGrantedPermission,
        receiver_user_id: TableId,
        permissions_id: TableId,
    ) -> Self {
        Self {
            giver_user_id: value.giver_user_id,
            receiver_user_id,
            permissions_id,
        }
    }
}

impl DbUpdateGrantedPermission {
    pub fn from_api(value: UpdateGrantedPermission, receiver_user_id: Option<TableId>) -> Self {
        Self {
            id: value.id,
            giver_user_id: None,
            receiver_user_id,
            permissions_id: None,
        }
    }
}
