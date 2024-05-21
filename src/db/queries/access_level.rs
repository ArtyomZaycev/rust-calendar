use crate::db::types::password::*;
use crate::error::Error;
use diesel::prelude::*;

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

pub fn db_insert_access_levels(
    connection: &mut MysqlConnection,
    new_access_levels: &Vec<DbNewAccessLevel>,
) -> Result<(), Error> {
    use crate::db::schema::access_levels::dsl::*;

    diesel::insert_into(access_levels)
        .values(new_access_levels)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_update_access_level(
    connection: &mut MysqlConnection,
    upd_access_level: &DbUpdateAccessLevel,
) -> Result<(), Error> {
    use crate::db::schema::access_levels::dsl::*;

    diesel::update(access_levels.find(upd_access_level.id))
        .set(upd_access_level)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_delete_access_levels_by_ids(
    connection: &mut MysqlConnection,
    ids: &Vec<i32>,
) -> Result<(), Error> {
    use crate::db::schema::access_levels::dsl as s;

    diesel::delete(s::access_levels.filter(s::id.eq_any(ids)))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;
    
    Ok(())
}
