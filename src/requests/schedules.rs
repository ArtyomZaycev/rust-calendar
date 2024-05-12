use calendar_lib::api::schedules::types::Schedule;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::schedule::{
            db_load_schedule_by_id, db_load_schedules_by_user_id_and_access_level_and_deleted,
        },
        session_info::SessionInfo,
    },
    error::Error,
};

use super::event_plans::load_event_plans_by_schedule_id;

pub fn load_session_schedule_by_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    id: i32,
) -> Result<Option<Schedule>, Error> {
    let schedule = db_load_schedule_by_id(connection, id)?;

    match schedule {
        Some(schedule) => {
            let permissions = session.get_permissions(schedule.user_id);
            if schedule.deleted
                || !permissions.schedules.view
                || permissions.access_level < schedule.access_level
            {
                Ok(None)
            } else {
                let event_plans = load_event_plans_by_schedule_id(connection, schedule.id)?;
                Ok(Some(schedule.to_api(event_plans)))
            }
        }
        None => Ok(None),
    }
}

pub fn load_session_schedules_by_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<Schedule>, Error> {
    let permissions = session.get_permissions(user_id);

    let schedules = if !permissions.schedules.view {
        Vec::default()
    } else {
        db_load_schedules_by_user_id_and_access_level_and_deleted(
            connection,
            user_id,
            permissions.access_level,
            false,
        )?
    };

    Ok(schedules
        .into_iter()
        .filter_map(|schedule| {
            let event_plans = load_event_plans_by_schedule_id(connection, schedule.id).ok()?;
            Some(schedule.to_api(event_plans))
        })
        .collect())
}
