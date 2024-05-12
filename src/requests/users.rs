use calendar_lib::api::utils::User;
use diesel::MysqlConnection;

use crate::{
    db::{queries::user::*, types::user::DbUser},
    error::Error,
};

use super::roles::load_user_roles;

pub fn fill_user_roles(connection: &mut MysqlConnection, user: DbUser) -> Result<User, Error> {
    let roles = load_user_roles(connection, user.id)?;
    Ok(user.to_api(roles))
}

pub fn load_user_by_id(connection: &mut MysqlConnection, id: i32) -> Result<Option<User>, Error> {
    match db_load_user_by_id(connection, id)? {
        Some(user) => Ok(Some(fill_user_roles(connection, user)?)),
        None => Ok(None),
    }
}

pub fn load_users(connection: &mut MysqlConnection) -> Result<Vec<User>, Error> {
    let users = db_load_users(connection)?;
    Ok(users
        .into_iter()
        .filter_map(|user| fill_user_roles(connection, user).ok())
        .collect())
}
