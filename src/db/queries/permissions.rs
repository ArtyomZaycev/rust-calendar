use crate::db::types::permission::*;
use crate::error::Error;
use diesel::prelude::*;

pub fn db_load_permission_by_id(
    connection: &mut MysqlConnection,
    id: i32,
) -> Result<Option<DbPermission>, Error> {
    use crate::db::schema::permissions::dsl as p;

    p::permissions
        .filter(p::id.eq(id))
        .load::<DbPermission>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_permissions_by_ids(
    connection: &mut MysqlConnection,
    ids: Vec<i32>,
) -> Result<Vec<DbPermission>, Error> {
    use crate::db::schema::permissions::dsl as p;

    p::permissions
        .filter(p::id.eq_any(ids))
        .load::<DbPermission>(connection)
        .map_err(|e| Error::DieselError(e))
}
