use calendar_lib::api::utils::User;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::user::{db_load_user_by_email, db_load_user_by_id},
        types::user::DbUser,
    },
    error::Error,
};

use super::roles::load_user_roles;

fn user_to_api(
    connection: &mut MysqlConnection,
    user: Option<DbUser>,
) -> Result<Option<User>, Error> {
    match user {
        Some(user) => {
            let roles = load_user_roles(connection, user.id)?;
            Ok(Some(user.to_api(roles)))
        }
        None => Ok(None),
    }
}

pub fn load_user_by_id(connection: &mut MysqlConnection, id: i32) -> Result<Option<User>, Error> {
    let user = db_load_user_by_id(connection, id)?;
    user_to_api(connection, user)
}

pub fn load_user_by_email(
    connection: &mut MysqlConnection,
    email: &str,
) -> Result<Option<User>, Error> {
    let user = db_load_user_by_email(connection, &email)?;
    user_to_api(connection, user)
}
