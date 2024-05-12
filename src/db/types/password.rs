use calendar_lib::api::auth::types::AccessLevel;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbPassword {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub password: String,
    pub access_level: i32,
    pub edit_right: bool,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::passwords)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewPassword {
    pub user_id: i32,
    pub name: String,
    pub password: String,
    pub access_level: i32,
    pub edit_right: bool,
}

impl Into<AccessLevel> for DbPassword {
    fn into(self) -> AccessLevel {
        AccessLevel {
            id: self.id,
            user_id: self.user_id,
            level: self.access_level,
            name: self.name,
        }
    }
}
