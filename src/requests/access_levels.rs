use calendar_lib::api::auth::types::AccessLevel;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::password::db_load_available_passwords, session_info::SessionInfo,
        types::password::DbPassword,
    },
    error::Error,
};

pub fn load_session_access_levels(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
) -> Result<Vec<AccessLevel>, Error> {
    let mut passwords = db_load_available_passwords(
        connection,
        session.get_user_id(),
        session.get_access_level(),
    )?;
    if !session.get_edit_rights() {
        passwords = passwords.into_iter().filter(|p| !p.edit_right).collect();
    }

    Ok(passwords.into_iter().map(DbPassword::into).collect())
}
