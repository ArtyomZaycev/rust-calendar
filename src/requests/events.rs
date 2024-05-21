use calendar_lib::api::events::types::Event;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::event::{db_load_event_by_id, db_load_events_by_user_id},
        session_info::SessionInfo,
    },
    error::Error,
};

pub fn load_session_event_by_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    id: i32,
) -> Result<Option<Event>, Error> {
    let event = db_load_event_by_id(connection, id)?;

    match event {
        Some(event) => {
            let permissions = session.get_permissions(event.user_id);
            if !permissions.events.view {
                Ok(None)
            } else {
                Ok(event.try_to_api(permissions.access_level))
            }
        }
        None => Ok(None),
    }
}

pub fn load_session_events_by_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<Event>, Error> {
    let permissions = session.get_permissions(user_id);

    let events = if !permissions.events.view {
        Vec::default()
    } else {
        // There's a complex check after
        db_load_events_by_user_id(connection, user_id)?
    };

    Ok(events
        .into_iter()
        .filter_map(|event| event.try_to_api(permissions.access_level))
        .collect())
}
