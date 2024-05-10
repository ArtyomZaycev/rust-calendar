use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{users::*, utils::{LoadByIdQuery, UnauthorizedResponse}};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::authenticate_request,
    db::queries::user::db_load_user_ids,
    error::InternalErrorWrapper,
    requests::users::{load_user_by_id, load_users},
    state::*,
};

pub async fn load_user_ids_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_ids::Args>,
) -> impl Responder {
    use load_ids::*;

    log_request_no_body("LoadUserIds", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.is_admin() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        let user_ids = db_load_user_ids(connection).internal()?;

        Ok(HttpResponse::Ok().json(Response { array: user_ids }))
    })
}

pub async fn load_user_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load::Args>,
) -> impl Responder {
    use load::*;

    log_request_no_body("LoadUser", &args);

    let LoadByIdQuery { id: user_id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.is_admin() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        match load_user_by_id(connection, user_id).internal()? {
            Some(user) => Ok(HttpResponse::Ok().json(user)),
            None => Err(HttpResponse::BadRequest().json(BadRequestResponse::NotFound)),
        }
    })
}

pub async fn load_users_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadUsers", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.is_admin() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }

        let users = load_users(connection).internal()?;

        Ok(HttpResponse::Ok().json(users))
    })
}
