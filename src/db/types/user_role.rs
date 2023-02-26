use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Clone, Serialize, Deserialize)]
pub struct DbUserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub granted: NaiveDateTime,
}

#[derive(diesel::Insertable, Clone)]
#[diesel(table_name = crate::db::schema::user_roles)]
pub struct DbNewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
