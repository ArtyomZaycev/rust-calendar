use actix_web::{web, HttpRequest, HttpResponse, Responder};

use calendar_lib::api::{
    auth::{types::AccessLevel, *},
    roles::types::Role,
    utils::UnauthorizedResponse,
};
use diesel::MysqlConnection;

use super::utils::*;
use crate::{
    db::{
        queries::{password::*, session::*, user::*},
        types::{password::{DbNewPassword, DbPassword}, session::DbNewSession, user::DbNewUser},
    },
    error::*,
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

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        invalidate_user_sessions(connection, session.user_id).internal()?;

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

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let user = load_user_by_email(connection, &email)
            .internal()?
            .ok_or(HttpResponse::BadRequest().body("User not found"))?;
        let passwords = load_passwords_by_user_id(connection, user.id).internal()?;
        let password = passwords
            .iter()
            .find(|pass| pass.password == password)
            .ok_or(HttpResponse::BadRequest().body("User not found"))?;

        let new_session = DbNewSession::new(password.id);
        insert_session(connection, &new_session).internal()?;

        Ok(HttpResponse::Ok().json(Response {
            user: user.into(),
            access_level: AccessLevel {
                level: password.access_level,
                name: password.name.clone(),
                edit_rights: password.edit_right,
            },
            key: new_session.key,
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

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        if exists_user_by_email(connection, &email).internal()? {
            Err(HttpResponse::BadRequest().json(BadRequestResponse::EmailAlreadyUsed))?
        }

        let user = insert_load_user(connection, &DbNewUser { name, email })
            .internal()?
            .internal()?;

        let new_password = DbNewPassword {
            user_id: user.id,
            name: "Full".to_owned(),
            password,
            access_level: 1000,
            edit_right: true,
        };

        insert_password(connection, &new_password).internal()?;

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
        editor_password
    } = body.0;

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if session.user_id != user_id && !session.has_role(Role::SuperAdmin) {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::Unauthorized))?;
        }
        if !session.is_max_acess_level() {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoAccessLevel))?;
        }
        if !session.edit_rights {
            Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoEditRights))?;
        }

        if access_level >= AccessLevel::MAX_LEVEL {
            Err(HttpResponse::BadRequest().finish())?;
        }
        if viewer_password.is_none() && editor_password.is_none() {
            Err(HttpResponse::BadRequest().finish())?;
        }

        push_passwords(connection, user_id, access_level).internal()?;
        if let Some(viewer_password) = viewer_password {
            insert_password(
                connection,
                &DbNewPassword {
                    name: viewer_password.name,
                    user_id: session.user_id,
                    password: viewer_password.password,
                    access_level,
                    edit_right: false,
                },
            )
            .internal()?;
        }
        if let Some(editor_password) = editor_password {
            insert_password(
                connection,
                &DbNewPassword {
                    name: editor_password.name,
                    user_id: session.user_id,
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

    let connection: &mut MysqlConnection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;

        let mut passwords = load_passwords_by_user_id_and_access_level(connection, session.user_id, session.access_level).internal()?;
        if !session.edit_rights {
            passwords = passwords.into_iter().filter(|p| !p.edit_right).collect();
        }

        Ok(HttpResponse::Ok().json(Response { array: passwords.into_iter().map(DbPassword::into).collect()}))
    })
}