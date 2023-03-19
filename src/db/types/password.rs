use calendar_lib::api::auth::types::AccessLevel;

#[derive(diesel::Queryable, Clone)]
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
pub struct DbNewPassword {
    pub user_id: i32,
    pub name: String,
    pub password: String,
    pub access_level: i32,
    pub edit_right: bool,
}

impl Into<AccessLevel> for DbPassword {
    fn into(self) -> AccessLevel {
        AccessLevel { level: self.access_level, name: self.name, edit_rights: self.edit_right }
    }
}