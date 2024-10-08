use crate::db::types::event::*;
use crate::error::Error;
use diesel::prelude::*;

pub fn db_load_event_by_id(
    connection: &mut MysqlConnection,
    id: i32,
) -> Result<Option<DbEvent>, Error> {
    use crate::db::schema::events::dsl as e;

    e::events
        .find(id)
        .load::<DbEvent>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_events_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbEvent>, Error> {
    use crate::db::schema::events::dsl::*;

    events
        .filter(user_id.eq(uid))
        .load::<DbEvent>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_insert_event(
    connection: &mut MysqlConnection,
    new_event: &DbNewEvent,
) -> Result<(), Error> {
    use crate::db::schema::events::dsl as e;
    diesel::insert_into(e::events)
        .values(new_event)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;
    Ok(())
}

#[allow(dead_code)]
pub fn db_insert_events(
    connection: &mut MysqlConnection,
    new_events: &[DbNewEvent],
) -> Result<(), Error> {
    use crate::db::schema::events::dsl as e;

    diesel::insert_into(e::events)
        .values(new_events)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_update_event(
    connection: &mut MysqlConnection,
    upd_event: &DbUpdateEvent,
) -> Result<(), Error> {
    use crate::db::schema::events::dsl as e;

    diesel::update(e::events.find(upd_event.id))
        .set(upd_event)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_delete_event(connection: &mut MysqlConnection, eid: i32) -> Result<(), Error> {
    use crate::db::schema::events::dsl as e;

    diesel::delete(e::events.find(eid))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
