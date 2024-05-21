use calendar_lib::api::schedules::types::EventPlan;
use diesel::MysqlConnection;

use crate::{db::queries::event_plan::db_load_event_plans_by_schedule_id, error::Error};

pub fn load_event_plans_by_schedule_id(
    connection: &mut MysqlConnection,
    schedule_id: i32,
) -> Result<Vec<EventPlan>, Error> {
    let events = db_load_event_plans_by_schedule_id(connection, schedule_id)?;
    Ok(events.into_iter().map(|v| v.to_api()).collect())
}
