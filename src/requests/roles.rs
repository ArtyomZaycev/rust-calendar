use calendar_lib::api::roles::types::Role;
use diesel::MysqlConnection;

use crate::{
    db::queries::role::{db_load_roles, db_load_roles_by_user_id},
    error::Error,
};

pub fn load_user_roles(connection: &mut MysqlConnection, user_id: i32) -> Result<Vec<Role>, Error> {
    let roles = db_load_roles_by_user_id(connection, user_id)?;
    Ok(roles.into_iter().filter_map(|v| v.try_to_api()).collect())
}

pub fn load_roles(connection: &mut MysqlConnection) -> Result<Vec<Role>, Error> {
    let roles = db_load_roles(connection)?;
    Ok(roles.into_iter().filter_map(|v| v.try_to_api()).collect())
}
