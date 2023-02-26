use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::session::*;
use crate::error::Error;

pub fn invalidate_user_sessions(connection: &mut MysqlConnection, uid: i32) -> Result<(), Error> {
    use crate::db::schema::sessions::dsl::*;

    diesel::delete(sessions.filter(user_id.eq(uid)).filter(valid.eq(true)))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn load_user_session(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Option<DbSession>, Error> {
    use crate::db::schema::sessions::dsl::*;
    use diesel::dsl::now;

    sessions
        .filter(user_id.eq(uid))
        .filter(valid.eq(true))
        .filter(end.ge(now))
        .load::<DbSession>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_session(
    connection: &mut MysqlConnection,
    new_session: &DbNewSession,
) -> Result<(), Error> {
    use crate::db::schema::sessions::dsl::*;

    diesel::update(sessions)
        .filter(user_id.eq(new_session.user_id))
        .set(valid.eq(false))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    diesel::insert_into(sessions)
        .values(new_session)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
