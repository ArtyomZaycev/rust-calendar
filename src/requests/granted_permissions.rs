use calendar_lib::api::permissions::types::GrantedPermission;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::{granted_permission::*, permissions::*},
        session_info::SessionInfo,
        types::granted_permission::DbGrantedPermission,
    },
    error::Error,
};

fn granted_permissions_to_api(
    connection: &mut MysqlConnection,
    granted_permissions: Vec<DbGrantedPermission>,
) -> Result<Vec<GrantedPermission>, Error> {
    let permissions_ids = granted_permissions
        .iter()
        .map(|gp| gp.permissions_id)
        .collect::<Vec<_>>();
    let permissions = db_load_permissions_by_ids(connection, permissions_ids)?;

    Ok(granted_permissions
        .into_iter()
        .filter_map(|gp| {
            permissions
                .iter()
                .find(|p| p.id == gp.permissions_id)
                .map(|p| gp.to_api(p.clone().to_api()))
        })
        .collect())
}

pub fn load_granted_permissions_by_receiver_user_id(
    connection: &mut MysqlConnection,
    user_id: i32,
) -> Result<Vec<GrantedPermission>, Error> {
    let granted_permissions = db_load_granted_permissions_by_receiver_user_id(connection, user_id)?;
    granted_permissions_to_api(connection, granted_permissions)
}

pub fn load_session_granted_permissions_by_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    id: i32,
) -> Result<Option<GrantedPermission>, Error> {
    match db_load_granted_permission_by_id(connection, id)? {
        Some(granted_permission) => {
            let permissions = session.get_permissions(granted_permission.giver_user_id)
                | session.get_permissions(granted_permission.receiver_user_id);
            if !permissions.allow_share {
                Ok(None)
            } else {
                match db_load_permission_by_id(connection, granted_permission.id)? {
                    Some(permission) => Ok(Some(granted_permission.to_api(permission.to_api()))),
                    None => Ok(None),
                }
            }
        }
        None => Ok(None),
    }
}

pub fn load_session_granted_permissions_by_receiver_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<GrantedPermission>, Error> {
    let permissions = session.get_permissions(user_id);
    if !permissions.allow_share {
        return Ok(Vec::new());
    }
    load_granted_permissions_by_receiver_user_id(connection, user_id)
}

pub fn load_session_granted_permissions_by_giver_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<GrantedPermission>, Error> {
    let permissions = session.get_permissions(user_id);
    if !permissions.allow_share {
        return Ok(Vec::new());
    }
    let granted_permissions = db_load_granted_permissions_by_giver_user_id(connection, user_id)?;
    granted_permissions_to_api(connection, granted_permissions)
}

pub fn load_session_granted_permissions_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<GrantedPermission>, Error> {
    let mut granted_permissions = vec![];
    granted_permissions.append(&mut load_session_granted_permissions_by_giver_user_id(
        connection, &session, user_id,
    )?);
    granted_permissions.append(&mut load_session_granted_permissions_by_receiver_user_id(
        connection, &session, user_id,
    )?);
    granted_permissions.dedup_by_key(|gp| gp.id);
    Ok(granted_permissions)
}
