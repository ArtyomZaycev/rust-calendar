use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{event_templates::*, utils::UnauthorizedResponse};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::*,
    db::{queries::event_template::*, types::event_template::*},
    error::InternalErrorWrapper,
    requests::event_templates::*,
    state::*,
};

pub async fn load_event_template_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadEventTemplate", &args);

    let id = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let event_template = load_session_event_template_by_id(connection, &session, id)
            .internal()?
            .ok_or(HttpResponse::BadRequest().json(BadRequestResponse::NotFound))?;
        Ok(HttpResponse::Ok().json(Response {
            value: event_template,
        }))
    })
}

pub async fn load_event_templates_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadEventTemplates", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let event_templates =
            load_session_event_templates_by_user_id(connection, &session, session.get_user_id())
                .internal()?;

        Ok(HttpResponse::Ok().json(event_templates))
    })
}

pub async fn insert_event_template_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<insert::Args>,
    body: web::Json<insert::Body>,
) -> impl Responder {
    use insert::*;

    log_request("InsertEventTemplate", &args, &body);

    let Args {} = args.0;
    let new_event_template = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session =
            authenticate_request_access(connection, req, true, new_event_template.access_level)?;

        if new_event_template.user_id != session.get_user_id() && !session.is_admin() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        db_insert_event_template(
            connection,
            &DbNewEventTemplate::from_api(new_event_template),
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn update_event_template_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<update::Args>,
    body: web::Json<update::Body>,
) -> impl Responder {
    use update::*;

    log_request("UpdateEventTemplate", &args, &body);

    let Args {} = args.0;
    let upd_event_template = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request_access(
            connection,
            req,
            true,
            upd_event_template.access_level.option_clone(),
        )?;

        if !session.get_edit_rights() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoEditRights))?;
        }

        if let Some(_) =
            load_session_event_template_by_id(connection, &session, upd_event_template.id)
                .internal()?
        {
            db_update_event_template(
                connection,
                &DbUpdateEventTemplate::from_api(upd_event_template),
            )
            .internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().finish())
        }
    })
}

pub async fn delete_event_template_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<delete::Args>,
) -> impl Responder {
    use delete::*;

    log_request_no_body("DeleteEventTemplate", &args);

    let id = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request_access(connection, req, true, None)?;

        if let Some(_) = load_session_event_template_by_id(connection, &session, id).internal()? {
            db_delete_event_template(connection, id).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Event template not found"))
        }
    })
}
