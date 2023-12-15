use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::other::{*, types::UserMemoryUsageData};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::authenticate_request,
    db::queries::{user::db_load_user_ids, event::db_load_events_by_user_id, password::db_load_passwords_by_user_id, schedule::db_load_schedules_by_user_id, event_plan::{db_load_event_plan_by_id, db_load_event_plans_by_schedule_id, db_load_event_plans_by_user_id}, event_template::db_load_event_templates_by_user_id},
    error::InternalErrorWrapper,
    requests::users::{load_user_by_id, load_users},
    state::*,
};

pub async fn load_user_memory_usage_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_user_memory_usage::Args>,
) -> impl Responder {
    use load_user_memory_usage::*;

    log_request_no_body("LoadUserMemoryUsage", &args);

    let Args { user_id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.is_admin() {
            Err(HttpResponse::BadRequest().finish())?;
        }

        let events = db_load_events_by_user_id(connection, user_id).internal()?;
        let passwords = db_load_passwords_by_user_id(connection, user_id).internal()?;
        let schedules = db_load_schedules_by_user_id(connection, user_id).internal()?;
        let event_plans = db_load_event_plans_by_user_id(connection, user_id).internal()?;
        let event_templates = db_load_event_templates_by_user_id(connection, user_id).internal()?;

        Ok(HttpResponse::Ok().json(Response { data: UserMemoryUsageData{
            events_count: events.len(),
            passwords_count: passwords.len(),
            schedules_count: schedules.len(),
            event_plans_count: event_plans.len(),
            event_templates_count: event_templates.len(),
            bytes: (
                events.len() * 281 +
                passwords.len() * 173 + 
                schedules.len() * 260 +
                event_plans.len() * 13 +
                event_templates.len() * 316
            ),
        } }))
    })
}
