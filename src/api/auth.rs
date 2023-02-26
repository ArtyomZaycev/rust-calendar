use actix_web::{web, HttpRequest, HttpResponse, Responder};

use calendar_lib::api::auth::{types::AccessLevel, *};

use super::utils::*;
use crate::{
    db::{
        queries::{password::*, session::*, user::*},
        types::{password::DbNewPassword, session::DbNewSession, user::DbNewUser},
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

    let connection = &mut data.pool.lock().unwrap();

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

    let connection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        let user = load_user_by_email(connection, &email)
            .internal()?
            .ok_or(HttpResponse::BadRequest().body("User not found"))?;
        let passwords = load_passwords_by_user_id(connection, user.id).internal()?;
        let password = passwords
            .iter()
            .find(|pass| pass.password == password)
            .ok_or(HttpResponse::BadRequest().body("User not found"))?;

        let new_session = DbNewSession::new(user.id, password.access_level, password.edit_right);
        insert_session(connection, &new_session).internal()?;

        Ok(HttpResponse::Ok().json(Response {
            user: user.into(),
            access_level: AccessLevel {
                level: password.access_level,
                name: password.name.clone(),
                edit_rights: new_session.edit_right,
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

    let connection = &mut data.pool.lock().unwrap();

    handle_request(|| {
        if exists_user_by_email(connection, &email).internal()? {
            Err(HttpResponse::BadRequest().json(BadRequestResponse::EmailAlreadyUsed))?
        }

        let user = insert_load_user(
            connection,
            &DbNewUser {
                name,
                email,
                phone: None,
            },
        )
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
        name,
        new_password,
        access_level,
        edit_right,
    } = body.0;

    let connection = &mut data.pool.lock().unwrap();
    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        if !session.full_access {
            Err(HttpResponse::Unauthorized().finish())?;
        }

        push_insert_password(
            connection,
            &DbNewPassword {
                name,
                user_id: session.user_id,
                password: new_password,
                access_level,
                edit_right,
            },
        )
        .internal()?;

        Ok(HttpResponse::Ok().json(Response {}))
    })
}
