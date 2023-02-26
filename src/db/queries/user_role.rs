use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::user_role::*;
use crate::error::Error;

pub fn load_user_role_by_id(
    connection: &mut MysqlConnection,
    urid: i32,
) -> Result<Option<DbUserRole>, Error> {
    use crate::db::schema::user_roles::dsl::*;

    user_roles
        .find(urid)
        .load::<DbUserRole>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn load_user_roles_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbUserRole>, Error> {
    use crate::db::schema::user_roles::dsl::*;

    user_roles
        .filter(user_id.eq(uid))
        .load::<DbUserRole>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_user_role(
    connection: &mut MysqlConnection,
    new_user_role: &DbNewUserRole,
) -> Result<(), Error> {
    use crate::db::schema::user_roles::dsl::*;

    diesel::insert_into(user_roles)
        .values(new_user_role)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn delete_user_role(connection: &mut MysqlConnection, urid: i32) -> Result<(), Error> {
    use crate::db::schema::user_roles::dsl::*;

    diesel::delete(user_roles.find(urid))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
