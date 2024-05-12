use calendar_lib::api::auth::types::AccessLevel;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbAccessLevel {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub level: i32,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::access_levels)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewAccessLevel {
    pub user_id: i32,
    pub name: String,
    pub level: i32,
}

impl DbAccessLevel {
    pub fn to_api(self) -> AccessLevel {
        AccessLevel {
            id: self.id,
            user_id: self.user_id,
            level: self.level,
            name: self.name,
        }
    }
}
