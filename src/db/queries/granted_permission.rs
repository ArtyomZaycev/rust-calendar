use crate::db::types::granted_permission::*;
use crate::error::Error;
use diesel::prelude::*;

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
