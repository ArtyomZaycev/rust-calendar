use crate::db::types::password::*;
use crate::error::Error;
use diesel::prelude::*;

pub fn db_load_access_levels_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbAccessLevel>, Error> {
    use crate::db::schema::access_levels::dsl::*;

    access_levels
        .filter(user_id.eq(uid))
        .load::<DbAccessLevel>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_access_levels_by_user_id_and_access_level(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
) -> Result<Vec<DbAccessLevel>, Error> {
    use crate::db::schema::access_levels::dsl::*;

    access_levels
        .filter(user_id.eq(uid))
        .filter(level.le(acc_level))
        .load::<DbAccessLevel>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_push_access_levels(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
) -> Result<(), Error> {
    use crate::db::schema::access_levels::dsl::*;

    diesel::update(access_levels)
        .filter(user_id.eq(uid))
        .filter(level.le(acc_level))
        .set(level.eq(level - 1))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_insert_access_level(
    connection: &mut MysqlConnection,
    new_access_level: &DbNewAccessLevel,
) -> Result<(), Error> {
    use crate::db::schema::access_levels::dsl::*;

    diesel::insert_into(access_levels)
        .values(new_access_level)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
