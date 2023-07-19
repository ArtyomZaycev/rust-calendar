use calendar_lib::api::auth::types::AccessLevel;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::password::{
            db_load_passwords_by_user_id, db_load_passwords_by_user_id_and_access_level,
            db_load_passwords_by_user_id_and_access_level_and_edit_rights,
        },
        session_info::SessionInfo,
        types::password::DbPassword,
    },
    error::Error,
};

pub fn load_session_access_levels_by_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<AccessLevel>, Error> {
    let passwords = if session.is_admin() {
        db_load_passwords_by_user_id(connection, user_id)?
    } else if session.get_user_id() == user_id {
        if session.get_edit_rights() {
            db_load_passwords_by_user_id_and_access_level(
                connection,
                session.get_user_id(),
                session.get_access_level(),
            )?
        } else {
            db_load_passwords_by_user_id_and_access_level_and_edit_rights(
                connection,
                session.get_user_id(),
                session.get_access_level(),
                false,
            )?
        }
    } else {
        Vec::default()
    };

    Ok(passwords.into_iter().map(DbPassword::into).collect())
}
