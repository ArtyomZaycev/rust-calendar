use crate::db::types::password::*;
use crate::error::Error;
use diesel::prelude::*;

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

pub fn load_available_passwords(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
) -> Result<Vec<DbPassword>, Error> {
    use crate::db::schema::passwords::dsl::*;

    passwords
        .filter(user_id.eq(uid))
        .filter(access_level.le(acc_level))
        .load::<DbPassword>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn load_passwords_by_user_id_and_access_level_and_edit_rights(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
    edit: bool,
) -> Result<Option<DbPassword>, Error> {
    use crate::db::schema::passwords::dsl::*;

    passwords
        .filter(user_id.eq(uid))
        .filter(access_level.eq(acc_level))
        .filter(edit_right.eq(edit))
        .load::<DbPassword>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn push_passwords(
    connection: &mut MysqlConnection,
    uid: i32,
    acc_level: i32,
) -> Result<(), Error> {
    use crate::db::schema::passwords::dsl::*;
    diesel::update(passwords)
        .filter(user_id.eq(uid))
        .filter(access_level.le(acc_level))
        .set(access_level.eq(access_level - 1))
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
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

pub fn insert_passwords(
    connection: &mut MysqlConnection,
    new_passwords: &[DbNewPassword],
) -> Result<(), Error> {
    use crate::db::schema::passwords::dsl::*;

    diesel::insert_into(passwords)
        .values(new_passwords)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}
