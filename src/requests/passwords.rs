use calendar_lib::api::auth::types::AccessLevel;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::password::{
            db_load_password_by_user_id_and_access_level_and_edit_rights,
            db_load_passwords_by_user_id,
        },
        session_info::SessionInfo,
        types::password::DbPassword,
    },
    error::Error,
};

fn password_to_api(
    _connection: &mut MysqlConnection,
    password: Option<DbPassword>,
) -> Result<Option<AccessLevel>, Error> {
    Ok(password.map(|password| AccessLevel {
        level: password.access_level,
        name: password.name,
        edit_rights: password.edit_right,
    }))
}

pub fn load_user_access_level(
    connection: &mut MysqlConnection,
    user_id: i32,
    password: &str,
) -> Result<Option<AccessLevel>, Error> {
    let passwords = db_load_passwords_by_user_id(connection, user_id)?;
    password_to_api(
        connection,
        passwords.into_iter().find(|pass| pass.password == password),
    )
}

pub fn load_session_access_level(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
) -> Result<Option<AccessLevel>, Error> {
    let password = db_load_password_by_user_id_and_access_level_and_edit_rights(
        connection,
        session.get_user_id(),
        session.get_access_level(),
        session.get_edit_rights(),
    )?;
    password_to_api(connection, password)
}
