use diesel::prelude::*;

use crate::db::types::user::*;
use crate::error::Error;

pub fn load_user_by_id(
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

pub fn load_user_by_email(
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

pub fn insert_user(connection: &mut MysqlConnection, new_user: &DbNewUser) -> Result<(), Error> {
    use crate::db::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(new_user)
        .execute(connection)
        .map_err(|e| Error::DieselError(e))?;

    Ok(())
}

pub fn insert_load_user(
    connection: &mut MysqlConnection,
    new_user: &DbNewUser,
) -> Result<Option<DbUser>, Error> {
    insert_user(connection, new_user).and_then(|_| load_user_by_email(connection, &new_user.email))
}
