use actix_web::{HttpRequest, HttpResponse};
use base64::decode;
use calendar_lib::api::utils::UnauthorizedResponse;
use diesel::MysqlConnection;
use std::fmt::Debug;

use crate::{
    db::{
        queries::{role::*, session::*},
        session_info::SessionInfo,
    },
    error::InternalErrorWrapper,
};

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

pub fn authenticate(
    connection: &mut MysqlConnection,
    user_id: i32,
    key: &[u8],
) -> Result<SessionInfo, HttpResponse> {
    let session = load_user_session(connection, user_id).internal()?;
    if let Some(session) = session {
        if session.key != key {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::WrongKey))?;
        }

        let roles = load_roles_by_user_id(connection, user_id).internal()?;
        Ok(SessionInfo {
            user_id: session.user_id,
            access_level: session.access_level,
            edit_rights: session.edit_right,
            roles,
        })
    } else {
        Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::WrongKey))
    }
}

pub fn authenticate_request(
    connection: &mut MysqlConnection,
    req: HttpRequest,
) -> Result<SessionInfo, HttpResponse> {
    let auth_info = req.headers().get("authorization").and_then(|auth| {
        auth.to_str().ok().and_then(|auth| {
            auth.starts_with("Basic ")
                .then(|| {
                    decode(&auth[6..]).ok().and_then(|decoded| {
                        decoded
                            .iter()
                            .position(|&c| c as char == ':')
                            .and_then(|pos| {
                                String::from_utf8_lossy(&decoded[..pos])
                                    .parse::<i32>()
                                    .ok()
                                    .map(|user_id| (user_id, decoded[pos + 1..].to_vec()))
                            })
                    })
                })
                .unwrap_or(None)
        })
    });

    auth_info.map_or(
        Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::WrongKey)),
        |(user_id, key)| authenticate(connection, user_id, &key),
    )
}

pub fn authenticate_request_access(
    connection: &mut MysqlConnection,
    req: HttpRequest,
    need_edit_right: bool,
    min_access_level: impl Into<Option<i32>>,
) -> Result<SessionInfo, HttpResponse> {
    let session = authenticate_request(connection, req)?;
    if need_edit_right && !session.edit_rights {
        Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoEditRights))?;
    }
    if let Some(min_access_level) = min_access_level.into() {
        if session.access_level < min_access_level {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel))?;
        }
    }
    Ok(session)
}
