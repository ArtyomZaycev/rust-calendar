use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{
    auth::{
        types::{AccessLevel, NewPassword},
        *,
    },
    roles::types::Role,
    utils::UnauthorizedResponse,
};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    api::{
        jwt::{create_jwt, jwt_to_string, CustomClaims},
        utils::*,
    },
    db::{
        queries::{password::*, session::*, user::*},
        types::{password::DbNewPassword, user::DbNewUser},
    },
    error::InternalErrorWrapper,
    requests::{access_levels::*, passwords::*, users::*},
    state::*,
};

pub async fn logout_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<logout::Args>,
    body: web::Json<logout::Body>,
) -> impl Responder {
    use logout::*;

    log_request("Logout", &args, &body);

    let Args {} = args.0;
    let Body {} = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        invalidate_user_sessions(connection, session.get_user_id()).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn login_handler(
    data: web::Data<AppState>,
    args: web::Query<login::Args>,
    body: web::Json<login::Body>,
) -> impl Responder {
    use login::*;

    log_request("Login", &args, &body);

    let Args {} = args.0;
    let Body { email, password } = body.0;
    let password = hash_password(&password);

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let user = load_user_by_email(connection, &email)
            .internal()?
            .ok_or(HttpResponse::BadRequest().json(BadRequestResponse::UserNotFound))?;
        let access_level = load_user_access_level(connection, user.id, &password)
            .internal()?
            .ok_or(HttpResponse::BadRequest().json(BadRequestResponse::UserNotFound))?;

        let jwt = create_jwt(CustomClaims {
            user_id: user.id,
            access_level: access_level.level,
            edit_rights: access_level.edit_rights,
        })
        .internal()?;

        //insert_session(connection, &new_session).internal()?;

        Ok(HttpResponse::Ok().json(Response {
            user,
            access_level,
            jwt,
        }))
    })
}

pub async fn login_by_key_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<login_by_key::Args>,
    body: web::Json<login_by_key::Body>,
) -> impl Responder {
    use login_by_key::*;

    log_request("LoginByKey", &args, &body);

    let Args {} = args.0;
    let Body {} = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        let user = load_user_by_id(connection, session.get_user_id())
            .internal()?
            // Internal bc request shouldn't have been authorized anyway
            .internal()?;
        let access_level = load_session_access_level(connection, &session)
            .internal()?
            // Internal bc request shouldn't have been authorized anyway
            .internal()?;

        Ok(HttpResponse::Ok().json(Response {
            user,
            access_level,
            jwt: jwt_to_string(session.jwt).internal()?,
        }))
    })
}

pub async fn register_handler(
    data: web::Data<AppState>,
    args: web::Query<register::Args>,
    body: web::Json<register::Body>,
) -> impl Responder {
    use register::*;

    log_request("Register", &args, &body);

    let Args {} = args.0;
    let Body {
        name,
        email,
        password,
    } = body.0;
    let password = hash_password(&password);

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        if exists_user_by_email(connection, &email).internal()? {
            Err(HttpResponse::BadRequest().json(BadRequestResponse::EmailAlreadyUsed))?
        }

        let user = db_insert_load_user(connection, &DbNewUser { name, email })
            .internal()?
            .internal()?;

        let new_password = DbNewPassword {
            user_id: user.id,
            name: "Full".to_owned(),
            password,
            access_level: AccessLevel::MAX_LEVEL,
            edit_right: true,
        };

        db_insert_password(connection, &new_password).internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn insert_password_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<new_password::Args>,
    body: web::Json<new_password::Body>,
) -> impl Responder {
    use new_password::*;

    log_request("NewPassword", &args, &body);

    let Args {} = args.0;
    let Body {
        user_id,
        access_level,
        viewer_password,
        editor_password,
    } = body.0;
    let viewer_password = viewer_password.map(|password| NewPassword {
        password: hash_password(&password.password),
        ..password
    });
    let editor_password = editor_password.map(|password| NewPassword {
        password: hash_password(&password.password),
        ..password
    });

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if session.get_user_id() != user_id && !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }
        if !session.is_max_acess_level() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel))?;
        }
        if !session.get_edit_rights() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoEditRights))?;
        }

        // TODO
        if access_level >= AccessLevel::MAX_LEVEL {
            Err(HttpResponse::BadRequest().finish())?;
        }
        if viewer_password.is_none() && editor_password.is_none() {
            Err(HttpResponse::BadRequest().finish())?;
        }

        // TODO: Move to a separate function
        db_push_passwords(connection, user_id, access_level).internal()?;
        if let Some(viewer_password) = viewer_password {
            db_insert_password(
                connection,
                &DbNewPassword {
                    name: viewer_password.name,
                    user_id: session.get_user_id(),
                    password: viewer_password.password,
                    access_level,
                    edit_right: false,
                },
            )
            .internal()?;
        }
        if let Some(editor_password) = editor_password {
            db_insert_password(
                connection,
                &DbNewPassword {
                    name: editor_password.name,
                    user_id: session.get_user_id(),
                    password: editor_password.password,
                    access_level,
                    edit_right: true,
                },
            )
            .internal()?;
        }

        Ok(HttpResponse::Ok().json(Response {}))
    })
}

pub async fn load_access_levels_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<load_access_levels::Args>,
) -> impl Responder {
    use load_access_levels::*;

    log_request_no_body("LoadAccessLevels", &args);

    let Args {} = args.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        let access_levels =
            load_session_access_levels_by_user_id(connection, &session, session.get_user_id())
                .internal()?;

        Ok(HttpResponse::Ok().json(Response {
            array: access_levels,
        }))
    })
}
