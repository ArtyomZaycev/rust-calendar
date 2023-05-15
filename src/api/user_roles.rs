use super::utils::*;
use crate::{
    db::{
        queries::{role::*, user_role::*},
        types::user_role::*,
    },
    error::InternalErrorWrapper,
    state::*,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{roles::types::*, user_roles::*, utils::UnauthorizedResponse};
use diesel::MysqlConnection;

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
        let user_id = user_id.unwrap_or(session.user_id);

        if user_id != session.user_id && !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::BadRequest().finish())?;
        }

        let roles = load_roles_by_user_id(connection, user_id).internal()?;

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

        insert_user_role(connection, &DbNewUserRole { user_id, role_id }).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn delete_user_role_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<delete::Args>,
    body: web::Json<delete::Body>,
) -> impl Responder {
    use delete::*;

    log_request("DeleteUserRole", &args, &body);

    let Args { id } = args.0;
    let Body {} = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        delete_user_role(connection, id).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}
