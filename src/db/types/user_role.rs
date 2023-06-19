use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbUserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::user_roles)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
