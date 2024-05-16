use calendar_lib::api::utils::User;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::{
            granted_permission::{
                db_load_granted_permissions_by_giver_user_id,
                db_load_granted_permissions_by_receiver_user_id,
            },
            user::*,
        },
        session_info::SessionInfo,
        types::user::DbUser,
    },
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

pub fn load_session_users_by_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<User>, Error> {
    let permissions = session.get_permissions(user_id);

    let users = if session.is_admin() {
        load_users(connection)?
    } else if permissions.allow_share {
        let granted_permissions = vec![
            db_load_granted_permissions_by_giver_user_id(connection, user_id)?,
            db_load_granted_permissions_by_receiver_user_id(connection, user_id)?,
        ]
        .concat();
        let users = db_load_users_by_ids(
            connection,
            granted_permissions
                .into_iter()
                .flat_map(|gp| vec![gp.giver_user_id, gp.receiver_user_id])
                .collect(),
        )?;
        users.into_iter().map(|u| u.to_api(vec![])).collect()
    } else {
        vec![]
    };

    Ok(users)
}
