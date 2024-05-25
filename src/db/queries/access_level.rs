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

pub fn db_invert_user_access_levels(
    connection: &mut MysqlConnection,
    user_id: i32,
) -> Result<(), Error> {
    use crate::db::schema::access_levels::dsl as al;

    diesel::update(al::access_levels.filter(al::user_id.eq(user_id)))
        .set(al::level.eq(al::level * -1))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_clear_referenced_access_level(
    connection: &mut MysqlConnection,
    user_id: i32,
    level: i32,
) -> Result<(), Error> {
    use crate::db::schema::event_templates::dsl as et;
    use crate::db::schema::events::dsl as e;
    use crate::db::schema::permissions::dsl as p;
    use crate::db::schema::schedules::dsl as s;

    use crate::db::schema::access_levels::dsl as al;

    let next_access_level: Option<DbAccessLevel> = al::access_levels
        .filter(al::user_id.eq(user_id))
        .filter(al::level.lt(level))
        .order(al::level.desc())
        .limit(1)
        .load::<DbAccessLevel>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))?;
    let next_access_level = if next_access_level.is_some() {
        next_access_level
    } else {
        al::access_levels
            .filter(al::user_id.eq(user_id))
            .filter(al::level.gt(level))
            .order(al::level.desc())
            .limit(1)
            .load::<DbAccessLevel>(connection)
            .map(|v| v.into_iter().nth(0))
            .map_err(|e| Error::DieselError(e))?
    };

    let next_access_level = match next_access_level {
        Some(next_access_level) => next_access_level,
        None => return Ok(()),
    };
    let next_access_level = next_access_level.level;

    diesel::update(
        e::events
            .filter(e::user_id.eq(user_id))
            .filter(e::access_level.eq(level)),
    )
    .set(e::access_level.eq(next_access_level))
    .execute(connection)
    .map_err(|e| Error::DieselError(e))?;
    diesel::update(
        et::event_templates
            .filter(et::user_id.eq(user_id))
            .filter(et::access_level.eq(level)),
    )
    .set(et::access_level.eq(next_access_level))
    .execute(connection)
    .map_err(|e| Error::DieselError(e))?;
    diesel::update(
        s::schedules
            .filter(s::user_id.eq(user_id))
            .filter(s::access_level.eq(level)),
    )
    .set(s::access_level.eq(next_access_level))
    .execute(connection)
    .map_err(|e| Error::DieselError(e))?;
    diesel::update(
        p::permissions
            .filter(p::user_id.eq(user_id))
            .filter(p::access_level.eq(level)),
    )
    .set(p::access_level.eq(next_access_level))
    .execute(connection)
    .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
