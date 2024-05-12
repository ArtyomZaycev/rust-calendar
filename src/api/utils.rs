use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse};
use calendar_lib::api::utils::{TableId, UnauthorizedResponse};
use diesel::MysqlConnection;
use sha2::{Digest, Sha512};

use super::jwt::verify_jwt;
use crate::{
    db::session_info::SessionInfo, error::InternalErrorWrapper, requests::roles::load_user_roles,
};

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
            let roles = load_user_roles(connection, jwt.custom.user_id).internal()?;
            Ok(SessionInfo::new(jwt, roles, HashMap::default()))
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
        |key: String| authenticate(connection, &key),
    )
}

pub fn authenticate_request_access(
    connection: &mut MysqlConnection,
    req: HttpRequest,
    user_id: TableId,
    min_access_level: i32,
) -> Result<SessionInfo, HttpResponse> {
    let session = authenticate_request(connection, req)?;
    let permissions = session.get_permissions(user_id);
    if permissions.access_level < min_access_level {
        Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel))?;
    }
    Ok(session)
}
