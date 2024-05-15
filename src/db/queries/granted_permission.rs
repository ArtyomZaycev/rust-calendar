use crate::db::types::granted_permission::*;
use crate::error::Error;
use diesel::prelude::*;

pub fn db_load_granted_permission_by_id(
    connection: &mut MysqlConnection,
    id: i32,
) -> Result<Option<DbGrantedPermission>, Error> {
    use crate::db::schema::granted_permissions::dsl as g;

    g::granted_permissions
        .find(id)
        .load::<DbGrantedPermission>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_granted_permissions_by_giver_user_id(
    connection: &mut MysqlConnection,
    user_id: i32,
) -> Result<Vec<DbGrantedPermission>, Error> {
    use crate::db::schema::granted_permissions::dsl as g;

    g::granted_permissions
        .filter(g::giver_user_id.eq(user_id))
        .load::<DbGrantedPermission>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_granted_permissions_by_receiver_user_id(
    connection: &mut MysqlConnection,
    user_id: i32,
) -> Result<Vec<DbGrantedPermission>, Error> {
    use crate::db::schema::granted_permissions::dsl as g;

    g::granted_permissions
        .filter(g::receiver_user_id.eq(user_id))
        .load::<DbGrantedPermission>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_insert_granted_permission(
    connection: &mut MysqlConnection,
    new_permission: &DbNewGrantedPermission,
) -> Result<(), Error> {
    use crate::db::schema::granted_permissions::dsl as g;

    diesel::insert_into(g::granted_permissions)
        .values(new_permission)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_update_granted_permission(
    connection: &mut MysqlConnection,
    upd_permission: &DbUpdateGrantedPermission,
) -> Result<(), Error> {
    use crate::db::schema::granted_permissions::dsl as g;

    diesel::update(g::granted_permissions.find(upd_permission.id))
        .set(upd_permission)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_delete_granted_permission(connection: &mut MysqlConnection, id: i32) -> Result<(), Error> {
    use crate::db::schema::granted_permissions::dsl as g;

    diesel::delete(g::granted_permissions.find(id))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
