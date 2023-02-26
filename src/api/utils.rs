use actix_web::{HttpRequest, HttpResponse};
use base64::decode;
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
            Err(HttpResponse::Unauthorized().finish())?;
        }

        let roles = load_roles_by_user_id(connection, user_id).internal()?;
        Ok(SessionInfo {
            user_id: session.user_id,
            access_level: session.access_level,
            edit_rights: session.edit_right,
            full_access: session.edit_right && session.access_level == 1000,
            roles,
        })
    } else {
        Err(HttpResponse::Unauthorized().finish())
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
        Err(HttpResponse::Unauthorized().finish()),
        |(user_id, key)| authenticate(connection, user_id, &key),
    )
}
