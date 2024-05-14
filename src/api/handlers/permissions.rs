use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{permissions::*, utils::UnauthorizedResponse};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::*, db::{queries::{granted_permission::{db_insert_granted_permission, db_load_granted_permission_by_id, db_update_granted_permission}, permissions::{db_insert_permission, db_update_permission}, user::db_load_user_by_email}, types::{granted_permission::{DbNewGrantedPermission, DbUpdateGrantedPermission}, permission::{DbNewPermission, DbUpdatePermission}}, utils::last_insert_id}, error::InternalErrorWrapper, requests::granted_permissions::*, state::*
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
pub async fn insert_granted_permission_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<insert::Args>,
    body: web::Json<insert::Body>,
) -> impl Responder {
    use insert::*;

    log_request("InsertGrantedPermission", &args, &body);

    let Args {} = args.0;
    let new_granted_permission = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(
            connection,
            req,
        )?;
        let permissions = session.get_permissions(new_granted_permission.giver_user_id);
        if !permissions.allow_share {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission))?;
        }

        let receiver = match db_load_user_by_email(connection, &new_granted_permission.receiver_email).internal()? {
            Some(u) => u,
            None => return Err(HttpResponse::BadRequest().json(BadRequestResponse::UserEmailNotFound)),
        };

        db_insert_permission(connection, &DbNewPermission::from_api(new_granted_permission.permissions)).internal()?;
        let permissions_id = last_insert_id(connection).internal()?;
        db_insert_granted_permission(connection, &DbNewGrantedPermission::from_api(new_granted_permission, receiver.id, permissions_id)).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn update_granted_permission_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<update::Args>,
    body: web::Json<update::Body>,
) -> impl Responder {
    use update::*;
    println!("update_granted_permission_handler");

    log_request("UpdateGrantedPermission", &args, &body);

    let Args {} = args.0;
    let upd_granted_permission = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        match load_session_granted_permissions_by_id(connection, &session, upd_granted_permission.id).internal()? {
            Some(_) => {
                let receiver = match upd_granted_permission.receiver_email.option_ref() {
                    Some(receiver_email) => {
                        match db_load_user_by_email(connection, &receiver_email).internal()? {
                            Some(u) => Some(u),
                            None => return Err(HttpResponse::BadRequest().json(BadRequestResponse::UserEmailNotFound)),
                        }
                    },
                    None => None,
                };

                let db_granted_permission = db_load_granted_permission_by_id(connection, upd_granted_permission.id).internal()?.unwrap();

                db_update_granted_permission(connection, &DbUpdateGrantedPermission::from_api(upd_granted_permission.clone(), receiver.map(|u| u.id))).internal()?;
                if let Some(permissions) = upd_granted_permission.permissions.option() {
                    db_update_permission(connection, &DbUpdatePermission::from_api(db_granted_permission.id, permissions)).internal()?;
                }

                Ok(HttpResponse::Ok().json(Response {}))
            }
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}
/*
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
