use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::schedule::*;
use crate::error::Error;

pub fn load_schedule_by_id(
    connection: &mut MysqlConnection,
    sid: i32,
) -> Result<Option<DbSchedule>, Error> {
    use crate::db::schema::schedules::dsl::*;

    schedules
        .find(sid)
        .load::<DbSchedule>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn load_schedules_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbSchedule>, Error> {
    use crate::db::schema::schedules::dsl::*;

    schedules
        .filter(user_id.eq(uid))
        .load::<DbSchedule>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_schedule(
    connection: &mut MysqlConnection,
    new_schedule: &DbNewSchedule,
) -> Result<(), Error> {
    use crate::db::schema::schedules::dsl::*;

    diesel::insert_into(schedules)
        .values(new_schedule)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn insert_schedules(
    connection: &mut MysqlConnection,
    new_schedules: &[DbNewSchedule],
) -> Result<(), Error> {
    use crate::db::schema::schedules::dsl::*;

    diesel::insert_into(schedules)
        .values(new_schedules)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn update_schedule(
    connection: &mut MysqlConnection,
    upd_schedule: &DbUpdateSchedule,
) -> Result<(), Error> {
    use crate::db::schema::schedules::dsl::*;

    diesel::update(schedules.find(upd_schedule.id))
        .set(upd_schedule)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn delete_event(connection: &mut MysqlConnection, eid: i32) -> Result<(), Error> {
    use crate::db::schema::schedules::dsl::*;

    diesel::delete(schedules.find(eid))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
