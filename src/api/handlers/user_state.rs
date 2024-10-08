use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::user_state::*;
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::authenticate_request,
    error::InternalErrorWrapper,
    requests::{
        access_levels::load_session_access_levels_by_user_id,
        event_templates::load_session_event_templates_by_user_id,
        events::load_session_events_by_user_id,
        granted_permissions::load_session_granted_permissions_user_id,
        schedules::load_session_schedules_by_user_id, users::load_session_users_by_user_id,
    },
    state::*,
};

pub async fn load_user_state_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadUserState", &args);

    let Args { user_id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        let users = load_session_users_by_user_id(connection, &session, user_id).internal()?;
        let access_levels =
            load_session_access_levels_by_user_id(connection, &session, user_id).internal()?;
        let events = load_session_events_by_user_id(connection, &session, user_id).internal()?;
        let event_templates =
            load_session_event_templates_by_user_id(connection, &session, user_id).internal()?;
        let schedules =
            load_session_schedules_by_user_id(connection, &session, user_id).internal()?;
        let granted_permissions =
            load_session_granted_permissions_user_id(connection, &session, user_id).internal()?;

        Ok(HttpResponse::Ok().json(Response {
            users,
            access_levels,
            events,
            event_templates,
            schedules,
            granted_permissions,
        }))
    })
}
