use calendar_lib::api::{events::types::Event, schedules::types::EventPlan};
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::{
            event::{db_load_event_by_id, db_load_events_by_user_id},
            event_plan::db_load_event_plans_by_schedule_id,
        },
        session_info::SessionInfo,
    },
    error::Error,
};

pub fn load_event_plan_by_id(
    connection: &mut MysqlConnection,
    id: i32,
) -> Result<Option<Event>, Error> {
    let event = db_load_event_by_id(connection, id)?;

    match event {
        Some(event) => Ok(event.try_to_api_full()),
        None => Ok(None),
    }
}

pub fn load_event_plans_by_schedule_id(
    connection: &mut MysqlConnection,
    schedule_id: i32,
) -> Result<Vec<EventPlan>, Error> {
    let events = db_load_event_plans_by_schedule_id(connection, schedule_id)?;
    Ok(events.into_iter().map(|v| v.to_api()).collect())
}

pub fn load_session_event_by_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    id: i32,
) -> Result<Option<Event>, Error> {
    let event = db_load_event_by_id(connection, id)?;

    match event {
        Some(event) => {
            if !session.is_admin() && event.user_id != session.get_user_id() {
                Ok(None)
            } else {
                Ok(event.try_to_api(session.get_access_level()))
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
    let events = if session.is_admin() {
        db_load_events_by_user_id(connection, user_id)?
    } else if session.get_user_id() == user_id {
        // There's a complex check after
        db_load_events_by_user_id(connection, user_id)?
    } else {
        Vec::default()
    };

    Ok(events
        .into_iter()
        .filter_map(|event| event.try_to_api(session.get_access_level()))
        .collect())
}
