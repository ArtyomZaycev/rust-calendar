use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{events::*, roles::types::Role, utils::UnauthorizedResponse};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    db::{queries::event::*, types::event::*},
    error::InternalErrorWrapper,
    state::*,
};

pub async fn load_event_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadEvent", &args);

    let Args { id } = args.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let event = load_event_by_id(connection, id).internal()?;

        match event {
            Some(event) => {
                if event.user_id != session.user_id && !session.has_role(Role::SuperAdmin) {
                    Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound))?;
                }

                match event.try_to_api(session.access_level) {
                    Some(event) => Ok(HttpResponse::Ok().json(Response { value: event })),
                    None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
                }
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}

pub async fn load_events_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadEvents", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let events = load_events_by_user_id(connection, session.user_id).internal()?;

        Ok(HttpResponse::Ok().json(Response {
            array: events
                .into_iter()
                .filter_map(|event| event.try_to_api(session.access_level))
                .collect(),
        }))
    })
}

pub async fn insert_event_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<insert::Args>,
    body: web::Json<insert::Body>,
) -> impl Responder {
    use insert::*;

    log_request("InsertEvent", &args, &body);

    let Args {} = args.0;
    let Body { new_event } = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request_access(connection, req, true, new_event.access_level)?;

        if new_event.user_id != session.user_id && !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        insert_event(connection, &DbNewEvent::from_api(new_event)).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn update_event_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<update::Args>,
    body: web::Json<update::Body>,
) -> impl Responder {
    use update::*;

    log_request("UpdateEvent", &args, &body);

    let Args {} = args.0;
    let Body { upd_event } = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request_access(
            connection,
            req,
            true,
            upd_event.access_level.option_clone(),
        )?;

        if upd_event
            .user_id
            .option_ref()
            .map(|&user_id| session.user_id != user_id)
            .unwrap_or_default()
            && !session.has_role(Role::SuperAdmin)
        {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        let old_event = load_event_by_id(connection, upd_event.id).internal()?;
        if let Some(old_event) = old_event {
            if session.access_level < old_event.access_level {
                Err(HttpResponse::BadRequest().finish())?;
            }
            if old_event.user_id != session.user_id && !session.has_role(Role::SuperAdmin) {
                Err(HttpResponse::BadRequest().finish())?;
            }

            update_event(connection, &DbUpdateEvent::from_api(upd_event)).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().finish())
        }
    })
}

pub async fn delete_event_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<delete::Args>,
    body: web::Json<delete::Body>,
) -> impl Responder {
    use delete::*;

    log_request("DeleteEvent", &args, &body);

    let Args { id } = args.0;
    let Body {} = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request_access(connection, req, true, None)?;

        let event = load_event_by_id(connection, id).internal()?;
        if let Some(event) = event {
            if session.access_level < event.access_level {
                Err(HttpResponse::BadRequest().finish())?;
            }
            if event.user_id != session.user_id && !session.has_role(Role::SuperAdmin) {
                Err(HttpResponse::BadRequest().finish())?;
            }

            delete_event(connection, id).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Event not found"))
        }
    })
}
