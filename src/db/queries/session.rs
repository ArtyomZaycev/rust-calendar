use diesel::prelude::*;

use crate::db::types::session::*;
use crate::error::Error;

pub fn invalidate_user_sessions(connection: &mut MysqlConnection, uid: i32) -> Result<(), Error> {
    use crate::db::schema::passwords::dsl as p;
    use crate::db::schema::sessions::dsl as s;

    diesel::delete(
        s::sessions
            .filter(s::password_id.eq_any(p::passwords.filter(p::user_id.eq(uid)).select(p::id))),
    )
    .execute(connection)
    .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn load_user_session(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Option<DbSession>, Error> {
    use crate::db::schema::passwords::dsl as p;
    use crate::db::schema::sessions::dsl as s;
    use diesel::dsl::now;

    s::sessions
        .left_join(p::passwords)
        .select(s::sessions::all_columns())
        .filter(p::user_id.eq(uid))
        .filter(s::valid.eq(true))
        .filter(s::end.ge(now))
        .load::<DbSession>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_session(
    connection: &mut MysqlConnection,
    new_session: &DbNewSession,
) -> Result<(), Error> {
    use crate::db::schema::passwords::dsl as p;
    use crate::db::schema::sessions::dsl as s;

    // TODO: Remove unwrap
    let uid: i32 = p::passwords
        .select(p::user_id)
        .find(new_session.password_id)
        .load::<i32>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))?
        .unwrap();

    diesel::update(s::sessions)
        .filter(s::password_id.eq_any(p::passwords.select(p::id).filter(p::user_id.eq(uid))))
        .set(s::valid.eq(false))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    diesel::insert_into(s::sessions)
        .values(new_session)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
