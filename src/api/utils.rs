use super::jwt::verify_jwt;
use crate::{
    db::{queries::role::*, session_info::SessionInfo},
    error::InternalErrorWrapper,
};
use actix_web::{HttpRequest, HttpResponse};
use calendar_lib::api::utils::UnauthorizedResponse;
use diesel::MysqlConnection;
use sha2::{Digest, Sha512};
use std::fmt::Debug;

pub fn log_request_no_body<Args>(handler: &str, args: &Args)
where
    Args: Debug,
{
    log::debug!("{handler};Args={args:?};Body=None");
}

pub fn log_request<Args, Body>(handler: &str, args: &Args, body: &Body)
where
    Args: Debug,
    Body: Debug,
{
    log::debug!("{handler};Args={args:?};Body={body:?}");
}

pub fn handle_request<F>(f: F) -> HttpResponse
where
    F: FnOnce() -> Result<HttpResponse, HttpResponse>,
{
    (f()).unwrap_or_else(|e| e)
}

pub fn hash_password(password: &str) -> String {
    base16ct::lower::encode_string(
        &Sha512::default()
            .chain_update(password.as_bytes())
            .finalize(),
    )
}

pub fn authenticate(
    connection: &mut MysqlConnection,
    jwt: &str,
) -> Result<SessionInfo, HttpResponse> {
    match verify_jwt(jwt) {
        Some(jwt) => {
            let roles = load_roles_by_user_id(connection, jwt.custom.user_id).internal()?;
            Ok(SessionInfo { jwt, roles })
        }
        None => Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::WrongKey)),
    }
}

pub fn authenticate_request(
    connection: &mut MysqlConnection,
    req: HttpRequest,
) -> Result<SessionInfo, HttpResponse> {
    let auth_info: Option<String> = req.headers().get("authorization").and_then(|auth| {
        auth.to_str()
            .ok()
            .and_then(|auth| auth.starts_with("Bearer ").then(|| auth[7..].to_owned()))
    });

    auth_info.map_or(
        Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::WrongKey)),
        |key| authenticate(connection, &key),
    )
}

pub fn authenticate_request_access(
    connection: &mut MysqlConnection,
    req: HttpRequest,
    need_edit_right: bool,
    min_access_level: impl Into<Option<i32>>,
) -> Result<SessionInfo, HttpResponse> {
    let session = authenticate_request(connection, req)?;
    if need_edit_right && !session.get_edit_rights() {
        Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoEditRights))?;
    }
    if let Some(min_access_level) = min_access_level.into() {
        if session.get_access_level() < min_access_level {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel))?;
        }
    }
    Ok(session)
}
