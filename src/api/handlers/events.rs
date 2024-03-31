use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{events::*, roles::types::Role, utils::UnauthorizedResponse};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::*,
    db::{queries::event::*, types::event::*},
    error::InternalErrorWrapper,
    requests::events::*,
    state::*,
};

pub async fn load_event_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadEvent", &args);

    let id = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let event = load_session_event_by_id(connection, &session, id).internal()?;

        match event {
            Some(event) => Ok(HttpResponse::Ok().json(event)),
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

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let events = load_session_events_by_user_id(connection, &session, session.get_user_id())
            .internal()?;

        Ok(HttpResponse::Ok().json(events))
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
    let new_event = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request_access(connection, req, true, new_event.access_level)?;

        if new_event.user_id != session.get_user_id() && !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        db_insert_event(connection, &DbNewEvent::from_api(new_event)).internal()?;

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
    let upd_event = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request_access(
            connection,
            req,
            true,
            upd_event.access_level.option_clone(),
        )?;

        if let Some(_) = load_session_event_by_id(connection, &session, upd_event.id).internal()? {
            db_update_event(connection, &DbUpdateEvent::from_api(upd_event)).internal()?;

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
) -> impl Responder {
    use delete::*;

    log_request_no_body("DeleteEvent", &args);

    let id = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request_access(connection, req, true, None)?;

        if let Some(_) = load_session_event_by_id(connection, &session, id).internal()? {
            db_delete_event(connection, id).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Event not found"))
        }
    })
}
