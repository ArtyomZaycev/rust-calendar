use calendar_lib::api::auth::types::AccessLevel;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::password::db_load_passwords_by_user_id_and_access_level,
        session_info::SessionInfo, types::password::DbPassword,
    },
    error::Error,
};

pub fn load_session_access_levels_by_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<AccessLevel>, Error> {
    let permissions = session.get_permissions(user_id);

    let passwords = if !permissions.access_levels.view {
        Vec::default()
    } else {
        db_load_passwords_by_user_id_and_access_level(
            connection,
            session.user_id,
            permissions.access_level,
        )?
    };

    Ok(passwords.into_iter().map(DbPassword::into).collect())
}
