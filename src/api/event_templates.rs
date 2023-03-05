use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::event_templates::*;
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    db::{queries::event_template::*, types::event_template::*},
    error::InternalErrorWrapper,
    state::*,
};

pub async fn load_event_template_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadEventTemplate", &args);

    let Args { id } = args.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let event_template = load_event_template_by_id(connection, id).internal()?;

        match event_template {
            Some(event_template) => {
                if event_template.user_id != session.user_id
                    || event_template.access_level > session.access_level
                {
                    Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound))
                } else {
                    Ok(HttpResponse::Ok().json(Response {
                        value: event_template.to_api(),
                    }))
                }
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
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

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let event_templates = load_event_templates_by_user_id_and_access_level(
            connection,
            session.user_id,
            session.access_level,
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {
            array: event_templates.into_iter().map(|v| v.to_api()).collect(),
        }))
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
    let Body { new_event_template } = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        if session.access_level < new_event_template.access_level || !session.edit_rights {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        insert_event_template(
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
    let Body { upd_event_template } = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let old_event = load_event_template_by_id(connection, upd_event_template.id).internal()?;
        if let Some(old_event) = old_event {
            if old_event.user_id != session.user_id || old_event.access_level > session.access_level
            {
                Err(HttpResponse::BadRequest().finish())?;
            }

            update_event_template(
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
    body: web::Json<delete::Body>,
) -> impl Responder {
    use delete::*;

    log_request("DeleteEventTemplate", &args, &body);

    let Args { id } = args.0;
    let Body {} = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.edit_rights {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        let event_template = load_event_template_by_id(connection, id).internal()?;
        if let Some(event_template) = event_template {
            if event_template.user_id != session.user_id
                || event_template.access_level > session.access_level
            {
                Err(HttpResponse::BadRequest().body("Event not found"))?;
            }

            delete_event_template(connection, id).internal()?;

            Ok(HttpResponse::Ok().json(Response {}))
        } else {
            Err(HttpResponse::BadRequest().body("Event not found"))
        }
    })
}
