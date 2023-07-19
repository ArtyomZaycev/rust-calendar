use calendar_lib::api::utils::User;
use diesel::MysqlConnection;

use crate::{
    db::{queries::user::*, types::user::DbUser},
    error::Error,
};

use super::roles::load_user_roles;

fn user_to_api(connection: &mut MysqlConnection, user: DbUser) -> Result<User, Error> {
    let roles = load_user_roles(connection, user.id)?;
    Ok(user.to_api(roles))
}

fn option_user_to_api(
    connection: &mut MysqlConnection,
    user: Option<DbUser>,
) -> Result<Option<User>, Error> {
    match user {
        Some(user) => Ok(Some(user_to_api(connection, user)?)),
        None => Ok(None),
    }
}

pub fn load_user_by_id(connection: &mut MysqlConnection, id: i32) -> Result<Option<User>, Error> {
    let user = db_load_user_by_id(connection, id)?;
    option_user_to_api(connection, user)
}

pub fn load_user_by_email(
    connection: &mut MysqlConnection,
    email: &str,
) -> Result<Option<User>, Error> {
    let user = db_load_user_by_email(connection, &email)?;
    option_user_to_api(connection, user)
}

pub fn load_users(connection: &mut MysqlConnection) -> Result<Vec<User>, Error> {
    let users = db_load_users(connection)?;
    Ok(users
        .into_iter()
        .filter_map(|v| user_to_api(connection, v).ok())
        .collect())
}
