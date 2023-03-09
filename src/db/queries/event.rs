use diesel::prelude::*;

use crate::db::types::event::*;
use crate::error::Error;

pub fn load_event_by_id(
    connection: &mut MysqlConnection,
    eid: i32,
) -> Result<Option<DbEvent>, Error> {
    use crate::db::schema::events::dsl::*;

    events
        .find(eid)
        .load::<DbEvent>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn load_events_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbEvent>, Error> {
    use crate::db::schema::events::dsl::*;

    events
        .filter(user_id.eq(uid))
        .load::<DbEvent>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn load_events_by_user_id_and_access_level(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
) -> Result<Vec<DbEvent>, Error> {
    use crate::db::schema::events::dsl::*;

    events
        .filter(user_id.eq(uid))
        .filter(access_level.le(acc_level))
        .load::<DbEvent>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_event(connection: &mut MysqlConnection, new_event: &DbNewEvent) -> Result<(), Error> {
    use crate::db::schema::events::dsl::*;

    diesel::insert_into(events)
        .values(new_event)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn insert_events(
    connection: &mut MysqlConnection,
    new_events: &[DbNewEvent],
) -> Result<(), Error> {
    use crate::db::schema::events::dsl::*;

    diesel::insert_into(events)
        .values(new_events)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn update_event(
    connection: &mut MysqlConnection,
    upd_event: &DbUpdateEvent,
) -> Result<(), Error> {
    use crate::db::schema::events::dsl::*;

    diesel::update(events.find(upd_event.id))
        .set(upd_event)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn delete_event(connection: &mut MysqlConnection, eid: i32) -> Result<(), Error> {
    use crate::db::schema::events::dsl::*;

    diesel::delete(events.find(eid))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
