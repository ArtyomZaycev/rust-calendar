use crate::db::types::user::*;
use crate::error::Error;
use diesel::prelude::*;

pub fn db_load_users(connection: &mut MysqlConnection) -> Result<Vec<DbUser>, Error> {
    use crate::db::schema::users::dsl::*;

    users
        .load::<DbUser>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_user_ids(connection: &mut MysqlConnection) -> Result<Vec<i32>, Error> {
    use crate::db::schema::users::dsl::*;

    users
        .select(id)
        .load::<i32>(connection)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_user_by_id(
    connection: &mut MysqlConnection,
    uid: i32,
) -> Result<Option<DbUser>, Error> {
    use crate::db::schema::users::dsl::*;

    users
        .find(uid)
        .load::<DbUser>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn exists_user_by_email(connection: &mut MysqlConnection, em: &str) -> Result<bool, Error> {
    use crate::db::schema::users::dsl::*;

    users
        .filter(email.eq(em))
        .count()
        .get_result(connection)
        .map(|c: i64| c > 0)
        .map_err(|e| Error::DieselError(e))
}

pub fn db_load_user_by_email(
    connection: &mut MysqlConnection,
    em: &str,
) -> Result<Option<DbUser>, Error> {
    use crate::db::schema::users::dsl::*;

    users
        .filter(email.eq(em))
        .load::<DbUser>(connection)
        .map(|v| v.into_iter().nth(0))
        .map_err(|e| Error::DieselError(e))
}

pub fn db_insert_user(connection: &mut MysqlConnection, new_user: &DbNewUser) -> Result<(), Error> {
    use crate::db::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn db_insert_load_user(
    connection: &mut MysqlConnection,
    new_user: &DbNewUser,
) -> Result<Option<DbUser>, Error> {
    db_insert_user(connection, new_user)
        .and_then(|_| db_load_user_by_email(connection, &new_user.email))
}
