use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::event_plan::*;
use crate::error::Error;

pub fn load_event_plan_by_id(
    connection: &mut MysqlConnection,
    eid: i32,
) -> Result<Option<DbEventPlan>, Error> {
    use crate::db::schema::event_plans::dsl::*;

    event_plans
        .find(eid)
        .load::<DbEventPlan>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn load_event_plans_by_schedule_id(
    connection: &mut MysqlConnection,
    sid: i32,
) -> Result<Vec<DbEventPlan>, Error> {
    use crate::db::schema::event_plans::dsl::*;

    event_plans
        .filter(schedule_id.eq(sid))
        .load::<DbEventPlan>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_event_plan(
    connection: &mut MysqlConnection,
    new_event_plan: &DbNewEventPlan,
) -> Result<(), Error> {
    use crate::db::schema::event_plans::dsl::*;

    diesel::insert_into(event_plans)
        .values(new_event_plan)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn insert_event_plans(
    connection: &mut MysqlConnection,
    new_event_plans: &[DbNewEventPlan],
) -> Result<(), Error> {
    use crate::db::schema::event_plans::dsl::*;

    diesel::insert_into(event_plans)
        .values(new_event_plans)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn delete_event_plan(connection: &mut MysqlConnection, eid: i32) -> Result<(), Error> {
    use crate::db::schema::event_plans::dsl::*;

    diesel::delete(event_plans.find(eid))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn delete_event_plans(connection: &mut MysqlConnection, eids: &[i32]) -> Result<(), Error> {
    use crate::db::schema::event_plans::dsl::*;

    diesel::delete(event_plans.filter(id.eq_any(eids)))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
