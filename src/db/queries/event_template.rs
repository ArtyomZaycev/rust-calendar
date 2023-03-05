use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::event_template::*;
use crate::error::Error;

pub fn load_event_template_by_id(
    connection: &mut MysqlConnection,
    eid: i32,
) -> Result<Option<DbEventTemplate>, Error> {
    use crate::db::schema::event_templates::dsl::*;

    event_templates
        .find(eid)
        .load::<DbEventTemplate>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn load_event_templates_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbEventTemplate>, Error> {
    use crate::db::schema::event_templates::dsl::*;

    event_templates
        .filter(user_id.eq(uid))
        .load::<DbEventTemplate>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn load_event_templates_by_user_id_and_access_level(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
) -> Result<Vec<DbEventTemplate>, Error> {
    use crate::db::schema::event_templates::dsl::*;

    event_templates
        .filter(user_id.eq(uid))
        .filter(access_level.le(acc_level))
        .load::<DbEventTemplate>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_event_template(
    connection: &mut MysqlConnection,
    new_event_template: &DbNewEventTemplate,
) -> Result<(), Error> {
    use crate::db::schema::event_templates::dsl::*;

    diesel::insert_into(event_templates)
        .values(new_event_template)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn insert_event_templates(
    connection: &mut MysqlConnection,
    new_event_templates: &[DbNewEventTemplate],
) -> Result<(), Error> {
    use crate::db::schema::event_templates::dsl::*;

    diesel::insert_into(event_templates)
        .values(new_event_templates)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn update_event_template(
    connection: &mut MysqlConnection,
    upd_event_template: &DbUpdateEventTemplate,
) -> Result<(), Error> {
    use crate::db::schema::event_templates::dsl::*;

    diesel::update(event_templates.find(upd_event_template.id))
        .set(upd_event_template)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn delete_event_template(connection: &mut MysqlConnection, eid: i32) -> Result<(), Error> {
    use crate::db::schema::event_templates::dsl::*;

    diesel::delete(event_templates.find(eid))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
