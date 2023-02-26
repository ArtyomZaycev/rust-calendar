use calendar_lib::api::roles::types::Role;
use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::role::*;
use crate::error::Error;

pub fn load_roles_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<Role>, Error> {
    use crate::db::schema::{roles::dsl::*, user_roles::dsl::*};

    roles
        .left_join(user_roles)
        .filter(user_id.eq(uid))
        .select(roles::all_columns())
        .load::<DbRole>(connection)
        .map(|v| {
            v.into_iter()
                .filter_map(|role: DbRole| role.try_into().ok())
                .collect()
        })
        .map_err(|e| Error::DieselError(e))
}

pub fn load_role_by_id(connection: &mut MysqlConnection, rid: i32) -> Result<Option<Role>, Error> {
    use crate::db::schema::roles::dsl::*;

    roles
        .find(rid)
        .load::<DbRole>(connection)
        .map(|v| {
            v.into_iter()
                .nth(0)
                .and_then(|role: DbRole| role.try_into().ok())
        })
        .map_err(|e| Error::DieselError(e))
}

pub fn load_roles(connection: &mut MysqlConnection) -> Result<Vec<Role>, Error> {
    use crate::db::schema::roles::dsl::*;

    roles
        .load::<DbRole>(connection)
        .map(|v| {
            v.into_iter()
                .filter_map(|role: DbRole| role.try_into().ok())
                .collect()
        })
        .map_err(|e| Error::DieselError(e))
}
