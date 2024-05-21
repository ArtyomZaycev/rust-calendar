use calendar_lib::api::event_templates::types::EventTemplate;
use diesel::MysqlConnection;

use crate::{
    db::{
        queries::event_template::{
            db_load_event_template_by_id, db_load_event_templates_by_user_id_and_access_level,
        },
        session_info::SessionInfo,
    },
    error::Error,
};

pub fn load_session_event_template_by_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    id: i32,
) -> Result<Option<EventTemplate>, Error> {
    let event_template = db_load_event_template_by_id(connection, id)?;

    match event_template {
        Some(event_template) => {
            let permissions = session.get_permissions(event_template.user_id);
            if !permissions.event_templates.view
                || permissions.access_level < event_template.access_level
            {
                Ok(None)
            } else {
                Ok(Some(event_template.to_api()))
            }
        }
        None => Ok(None),
    }
}

pub fn load_session_event_templates_by_user_id(
    connection: &mut MysqlConnection,
    session: &SessionInfo,
    user_id: i32,
) -> Result<Vec<EventTemplate>, Error> {
    let permissions = session.get_permissions(user_id);

    let event_templates = if !permissions.event_templates.view {
        Vec::default()
    } else {
        db_load_event_templates_by_user_id_and_access_level(
            connection,
            user_id,
            permissions.access_level,
        )?
    };

    Ok(event_templates.into_iter().map(|v| v.to_api()).collect())
}
