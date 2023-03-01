use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::schedules::*;
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    db::{
        queries::{
            event_plan::{insert_event_plans, load_event_plans_by_schedule_id},
            schedule::*,
        },
        types::{event_plan::DbNewEventPlan, schedule::DbNewSchedule},
        utils::last_insert_id,
    },
    error::InternalErrorWrapper,
    state::*,
};

pub async fn load_schedules_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadSchedules", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let schedules = load_schedules_by_user_id_and_access_level(
            connection,
            session.user_id,
            session.access_level,
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {
            array: schedules
                .into_iter()
                .filter_map(|schedule| {
                    let event_plans = load_event_plans_by_schedule_id(connection, schedule.id)
                        .ok()?
                        .into_iter()
                        .map(|v| v.to_api())
                        .collect();
                    Some(schedule.to_api(event_plans))
                })
                .collect(),
        }))
    })
}

pub async fn insert_schedule_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<insert::Args>,
    body: web::Json<insert::Body>,
) -> impl Responder {
    use insert::*;

    log_request("InsertSchedule", &args, &body);

    let Args {} = args.0;
    let Body { mut new_schedule } = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        if session.access_level < new_schedule.access_level || !session.edit_rights {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        let new_event_plans = new_schedule.events;
        new_schedule.events = vec![];

        insert_schedule(connection, &DbNewSchedule::from_api(new_schedule)).internal()?;
        let schedule_id = last_insert_id(connection).internal()?;

        insert_event_plans(
            connection,
            &new_event_plans
                .into_iter()
                .map(|event_plan| DbNewEventPlan::from_api(event_plan, schedule_id as i32))
                .collect::<Vec<_>>(),
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn delete_schedule_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<delete::Args>,
    body: web::Json<delete::Body>,
) -> impl Responder {
    use delete::*;

    log_request("DeleteSchedule", &args, &body);

    let Args { id } = args.0;
    let Body {} = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.edit_rights {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        let schedule = load_schedule_by_id(connection, id).internal()?;
        if let Some(schedule) = schedule {
            if schedule.user_id != session.user_id || schedule.access_level > session.access_level {
                Err(HttpResponse::BadRequest().body("Schedule not found"))?;
            }

            delete_schedule(connection, id).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Schedule not found"))
        }
    })
}
