use calendar_lib::api::utils::User;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::users)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbNewUser {
    pub name: String,
    pub email: String,
}

impl From<User> for DbUser {
    fn from(value: User) -> Self {
        DbUser {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}
impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        User {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}
