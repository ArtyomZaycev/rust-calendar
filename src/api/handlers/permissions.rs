use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::permissions::*;
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::*, error::InternalErrorWrapper, requests::granted_permissions::*, state::*,
};

pub async fn load_granted_permissions_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadGrantedPermissions", &args);

    let Args { user_id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let granted_permissions =
            load_session_granted_permissions_user_id(connection, &session, user_id).internal()?;
        Ok(HttpResponse::Ok().json(granted_permissions))
    })
}
/*
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
        let session = authenticate_request_access(
            connection,
            req,
            new_event.user_id,
            new_event.access_level,
        )?;
        let permissions = session.get_permissions(new_event.user_id);
        if !permissions.events.create {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission))?;
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
        let session = authenticate_request(connection, req)?;

        match load_session_event_by_id(connection, &session, upd_event.id).internal()? {
            Some(event) => {
                let permissions = session.get_permissions(event.user_id);
                if !permissions.events.edit {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission)
                    );
                }
                if permissions.access_level < upd_event.access_level.option_clone().unwrap_or(-1) {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel)
                    );
                }

                db_update_event(connection, &DbUpdateEvent::from_api(upd_event)).internal()?;

                Ok(HttpResponse::Ok().json(Response {}))
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
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

    let DeleteByIdQuery { id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        match load_session_event_by_id(connection, &session, id).internal()? {
            Some(event) => {
                let permissions = session.get_permissions(event.user_id);
                if !permissions.events.delete {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission)
                    );
                }
                if permissions.access_level < event.access_level {
                    return Err(
                        HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel)
                    );
                }

                db_delete_event(connection, id).internal()?;
                Ok(HttpResponse::Ok().json(Response {}))
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}
 */
