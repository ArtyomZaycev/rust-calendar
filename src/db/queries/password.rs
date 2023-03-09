use diesel::prelude::*;
use diesel::{MysqlConnection, QueryDsl, RunQueryDsl};

use crate::db::types::password::*;
use crate::error::Error;

pub fn load_password_by_id(
    connection: &mut MysqlConnection,
    pid: i32,
) -> Result<Option<DbPassword>, Error> {
    use crate::db::schema::passwords::dsl::*;

    passwords
        .find(pid)
        .load::<DbPassword>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn load_passwords_by_user_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Vec<DbPassword>, Error> {
    use crate::db::schema::passwords::dsl::*;

    passwords
        .filter(user_id.eq(uid))
        .load::<DbPassword>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn insert_password(
    connection: &mut MysqlConnection,
    new_password: &DbNewPassword,
) -> Result<(), Error> {
    use crate::db::schema::passwords::dsl::*;

    diesel::insert_into(passwords)
        .values(new_password)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn push_insert_password(
    connection: &mut MysqlConnection,
    new_password: &DbNewPassword,
) -> Result<(), Error> {
    {
        use crate::db::schema::events::dsl::*;

        diesel::update(events)
            .filter(user_id.eq(new_password.user_id))
            .filter(access_level.ge(new_password.access_level))
            .set(access_level.eq(access_level + 1))
            .execute(connection)
            .map_err(|e| Error::DieselError(e))?;
    };

    use crate::db::schema::passwords::dsl::*;
    diesel::update(passwords)
        .filter(user_id.eq(new_password.user_id))
        .filter(access_level.ge(new_password.access_level))
        .set(access_level.eq(access_level + 1))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    diesel::insert_into(passwords)
        .values(new_password)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn insert_passwords(
    connection: &mut MysqlConnection,
    new_passwords: &Vec<DbNewPassword>,
) -> Result<(), Error> {
    use crate::db::schema::passwords::dsl::*;

    diesel::insert_into(passwords)
        .values(new_passwords)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
