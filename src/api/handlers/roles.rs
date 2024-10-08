use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::roles::*;
use diesel::MysqlConnection;

use super::utils::*;
use crate::{api::utils::*, error::InternalErrorWrapper, requests::roles::load_roles, state::*};

pub async fn load_roles_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadRoles", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        authenticate_request(connection, req)?;
        let roles = load_roles(connection).internal()?;

        Ok(HttpResponse::Ok().json(roles))
    })
}
