use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::events::{types::*, *};

use super::utils::*;
use crate::{db::queries::event::*, error::InternalErrorWrapper, state::*};

pub async fn load_events_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadEvents", &args);

    let Args {} = args.0;

    let connection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let events = load_events_by_user_id_and_access_level(
            connection,
            session.user_id,
            session.access_level,
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {
            array: events.into_iter().map(Event::from).collect(),
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

    let connection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        if session.access_level < new_event.access_level || !session.edit_rights {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        insert_event(connection, &(session.user_id, new_event).into()).internal()?;

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

    let connection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let old_event = load_event_by_id(connection, upd_event.id).internal()?;
        if let Some(old_event) = old_event {
            if old_event.user_id != session.user_id || old_event.access_level > session.access_level
            {
                Err(HttpResponse::BadRequest().body("Event not found"))?;
            }

            update_event(connection, &upd_event.into()).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Event not found"))
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

    let connection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.edit_rights {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        let event = load_event_by_id(connection, id).internal()?;
        if let Some(event) = event {
            if event.user_id != session.user_id || event.access_level > session.access_level {
                Err(HttpResponse::BadRequest().body("Event not found"))?;
            }

            delete_event(connection, id).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Event not found"))
        }
    })
}
