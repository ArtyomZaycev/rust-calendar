use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::users::*;
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::utils::authenticate_request, error::InternalErrorWrapper,
    requests::users::load_session_users_by_user_id, state::*,
};

pub async fn load_users_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_array::Args>,
) -> impl Responder {
    use load_array::*;

    log_request_no_body("LoadUsers", &args);

    let Args { user_id } = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let users = load_session_users_by_user_id(connection, &session, user_id).internal()?;

        Ok(HttpResponse::Ok().json(users))
    })
}
