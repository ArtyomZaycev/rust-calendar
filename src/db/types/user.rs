use calendar_lib::api::utils::User;

#[derive(diesel::Queryable, Clone)]
pub struct DbUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(diesel::Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct DbNewUser {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

impl From<User> for DbUser {
    fn from(value: User) -> Self {
        DbUser {
            id: value.id,
            name: value.name,
            email: value.email,
            phone: value.phone,
        }
    }
}
impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        User {
            id: value.id,
            name: value.name,
            email: value.email,
            phone: value.phone,
        }
    }
}
