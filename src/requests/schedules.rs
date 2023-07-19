use calendar_lib::api::{auth::types::AccessLevel, schedules::types::Schedule};
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::schedule::{
            db_load_schedule_by_id, db_load_schedules_by_user_id,
            db_load_schedules_by_user_id_and_access_level_and_deleted,
        },
        session_info::SessionInfo,
    },
    error::Error,
};

use super::event_plans::load_event_plans_by_schedule_id;

pub fn load_schedule_by_id(
    connection: &mut MysqlConnection,
    id: i32,
) -> Result<Option<Schedule>, Error> {
    let schedule = db_load_schedule_by_id(connection, id)?;

    match schedule {
        Some(schedule) => {
            let event_plans = load_event_plans_by_schedule_id(connection, schedule.id)?;
            Ok(Some(schedule.to_api(event_plans)))
        }
        None => Ok(None),
    }
}

pub fn load_schedules_by_user_id(
    connection: &mut MysqlConnection,
    user_id: i32,
) -> Result<Vec<Schedule>, Error> {
    let schedules = db_load_schedules_by_user_id(connection, user_id)?;
    Ok(schedules
        .into_iter()
        .filter_map(|schedule| {
            let event_plans = load_event_plans_by_schedule_id(connection, schedule.id).ok()?;
            Some(schedule.to_api(event_plans))
        })
        .collect())
}

pub fn load_session_schedule_by_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    id: i32,
) -> Result<Option<Schedule>, Error> {
    let schedule = db_load_schedule_by_id(connection, id)?;

    match schedule {
        Some(schedule) => {
            if schedule.deleted
                || session.get_access_level() < schedule.access_level
                || (schedule.user_id != session.get_user_id() && !session.is_admin())
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
    let schedules = if session.is_admin() {
        db_load_schedules_by_user_id_and_access_level_and_deleted(
            connection,
            user_id,
            AccessLevel::MAX_LEVEL,
            false,
        )?
    } else if session.get_user_id() == user_id {
        db_load_schedules_by_user_id_and_access_level_and_deleted(
            connection,
            user_id,
            session.get_access_level(),
            false,
        )?
    } else {
        Vec::default()
    };

    Ok(schedules
        .into_iter()
        .filter_map(|schedule| {
            let event_plans = load_event_plans_by_schedule_id(connection, schedule.id).ok()?;
            Some(schedule.to_api(event_plans))
        })
        .collect())
}
