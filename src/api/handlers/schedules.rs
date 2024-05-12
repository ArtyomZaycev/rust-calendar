use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{
    schedules::*,
    utils::{DeleteByIdQuery, LoadByIdQuery, UnauthorizedResponse},
};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::*,
    db::{
        queries::{event_plan::*, schedule::*},
        types::{
            event_plan::DbNewEventPlan,
            schedule::{DbNewSchedule, DbUpdateSchedule},
        },
        utils::last_insert_id,
    },
    error::InternalErrorWrapper,
    requests::schedules::{load_session_schedule_by_id, load_session_schedules_by_user_id},
    state::*,
};

pub async fn load_schedule_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadSchedule", &args);

    let LoadByIdQuery { id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        match load_session_schedule_by_id(connection, &session, id).internal()? {
            Some(schedule) => Ok(HttpResponse::Ok().json(schedule)),
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}

pub async fn load_schedules_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadSchedules", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let schedules =
            load_session_schedules_by_user_id(connection, &session, session.user_id).internal()?;

        Ok(HttpResponse::Ok().json(schedules))
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
    let mut new_schedule = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request_access(
            connection,
            req,
            new_schedule.user_id,
            new_schedule.access_level,
        )?;
        let permissions = session.get_permissions(new_schedule.user_id);
        if !permissions.schedules.create {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission))?;
        }

        let new_event_plans = new_schedule.events;
        new_schedule.events = vec![];

        db_insert_schedule(connection, &DbNewSchedule::from_api(new_schedule)).internal()?;
        let schedule_id = last_insert_id(connection).internal()?;

        db_insert_event_plans(
            connection,
            &new_event_plans
                .into_iter()
                .map(|event_plan| DbNewEventPlan::from_api(event_plan, schedule_id))
                .collect::<Vec<_>>(),
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn update_schedule_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<update::Args>,
    body: web::Json<update::Body>,
) -> impl Responder {
    use update::*;

    log_request("UpdateSchedule", &args, &body);

    let Args {} = args.0;
    let upd_schedule = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        match load_session_schedule_by_id(connection, &session, upd_schedule.id).internal()? {
            Some(old_schedule) => {
                let permissions = session.get_permissions(old_schedule.user_id);
                if !permissions.schedules.edit {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission)
                    );
                }
                if permissions.access_level < upd_schedule.access_level.option_clone().unwrap_or(-1)
                {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel)
                    );
                }

                let event_plans = old_schedule.event_plans;

                let del_event_plans = &upd_schedule
                    .delete_events
                    .iter()
                    .filter_map(|&event_plan_id| {
                        event_plans
                            .iter()
                            .any(|e| e.id == event_plan_id)
                            .then_some(event_plan_id)
                    })
                    .collect::<Vec<_>>();
                db_delete_event_plans(connection, del_event_plans).internal()?;

                let new_event_plans = upd_schedule
                    .new_events
                    .iter()
                    .map(|new_event_plan| {
                        DbNewEventPlan::from_api(new_event_plan.clone(), upd_schedule.id)
                    })
                    .collect::<Vec<_>>();
                db_insert_event_plans(connection, &new_event_plans).internal()?;

                db_update_schedule(connection, &DbUpdateSchedule::from_api(upd_schedule))
                    .internal()?;

                Ok(HttpResponse::Ok().json(Response {}))
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}

pub async fn delete_schedule_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<delete::Args>,
) -> impl Responder {
    use delete::*;

    log_request_no_body("DeleteSchedule", &args);

    let DeleteByIdQuery { id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        match load_session_schedule_by_id(connection, &session, id).internal()? {
            Some(schedule) => {
                let permissions = session.get_permissions(schedule.user_id);
                if !permissions.schedules.delete {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission)
                    );
                }
                if permissions.access_level < schedule.access_level {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel)
                    );
                }

                db_delete_schedule(connection, id).internal()?;

                Ok(HttpResponse::Ok().json(Response {}))
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}
