use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{roles::types::*, user_roles::*, utils::UnauthorizedResponse};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::authenticate_request,
    db::{queries::user_role::*, types::user_role::*},
    error::InternalErrorWrapper,
    requests::roles::load_user_roles,
    state::*,
};

pub async fn load_user_roles_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadUserRoles", &args);

    let Args { user_id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let user_id = user_id.unwrap_or(session.get_user_id());

        if user_id != session.get_user_id() && !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::BadRequest().finish())?;
        }

        let roles = load_user_roles(connection, user_id).internal()?;

        Ok(HttpResponse::Ok().json(Response { array: roles }))
    })
}

pub async fn insert_user_role_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<insert::Args>,
    body: web::Json<insert::Body>,
) -> impl Responder {
    use insert::*;

    log_request("InsertUserRole", &args, &body);

    let Args {} = args.0;
    let Body { user_id, role_id } = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        if !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        db_insert_user_role(connection, &DbNewUserRole { user_id, role_id }).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn delete_user_role_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<delete::Args>,
) -> impl Responder {
    use delete::*;

    log_request_no_body("DeleteUserRole", &args);

    let Args { id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        db_delete_user_role(connection, id).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}
