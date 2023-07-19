use crate::db::types::role::*;
use crate::error::Error;

use diesel::prelude::*;

pub fn db_load_roles_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbRole>, Error> {
    use crate::db::schema::{roles::dsl::*, user_roles::dsl::*};

    roles
        .left_join(user_roles)
        .filter(user_id.eq(uid))
        .select(roles::all_columns())
        .load::<DbRole>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_role_by_id(
    connection: &mut MysqlConnection,
    rid: i32,
) -> Result<Option<DbRole>, Error> {
    use crate::db::schema::roles::dsl::*;

    roles
        .find(rid)
        .load::<DbRole>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_roles(connection: &mut MysqlConnection) -> Result<Vec<DbRole>, Error> {
    use crate::db::schema::roles::dsl::*;

    roles
        .load::<DbRole>(connection)
        .map_err(|e| Error::DieselError(e))
}
