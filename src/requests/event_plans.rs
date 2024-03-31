use calendar_lib::api::{events::types::Event, schedules::types::EventPlan};
use diesel::MysqlConnection;

use crate::{
    db::queries::{event::db_load_event_by_id, event_plan::db_load_event_plans_by_schedule_id},
    error::Error,
};

#[allow(dead_code)]
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
