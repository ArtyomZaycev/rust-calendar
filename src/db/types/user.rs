use calendar_lib::api::{roles::types::Role, utils::User};
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl DbUser {
    pub fn to_api(self, roles: Vec<Role>) -> User {
        User {
            id: self.id,
            name: self.name,
            email: self.email,
            roles,
        }
    }
}
