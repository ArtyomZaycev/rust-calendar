use actix_web::{web, HttpRequest, HttpResponse, Responder};
use calendar_lib::api::{
    auth::{types::AccessLevel, *},
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
        queries::{access_level::*, user::*},
        types::{
            password::{DbNewAccessLevel, DbUpdateAccessLevel},
            user::DbNewUser,
        },
    },
    error::InternalErrorWrapper,
    requests::{access_levels::*, users::*},
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
        let _session = authenticate_request(connection, req)?;

        //invalidate_user_sessions(connection, session.user_id).internal()?;

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
        let user = db_load_user_by_email(connection, &email)
            .internal()?
            .ok_or(HttpResponse::BadRequest().json(BadRequestResponse::UserNotFound))?;
        if password != user.password {
            return Err(HttpResponse::BadRequest().json(BadRequestResponse::UserNotFound));
        }

        let user = fill_user_roles(connection, user).internal()?;

        let jwt = create_jwt(CustomClaims {
            user_id: user.id,
            email: user.email.clone(),
        })
        .internal()?;

        //insert_session(connection, &new_session).internal()?;

        Ok(HttpResponse::Ok().json(Response { user, jwt }))
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

        let user = load_user_by_id(connection, session.user_id)
            .internal()?
            // Internal bc request shouldn't have been authorized anyway
            .internal()?;

        Ok(HttpResponse::Ok().json(Response {
            user,
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

        let user = db_insert_load_user(
            connection,
            &DbNewUser {
                name,
                email,
                password,
            },
        )
        .internal()?
        .internal()?;

        let new_access_level = DbNewAccessLevel {
            user_id: user.id,
            name: "Full".to_owned(),
            level: AccessLevel::MAX_LEVEL,
        };

        db_insert_access_level(connection, &new_access_level).internal()?;

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
            load_session_access_levels_by_user_id(connection, &session, session.user_id)
                .internal()?;

        Ok(HttpResponse::Ok().json(access_levels))
    })
}

pub async fn change_access_levels_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    args: web::Query<change_access_levels::Args>,
    body: web::Json<change_access_levels::Body>,
) -> impl Responder {
    use change_access_levels::*;

    log_request("ChangeAccessLevels", &args, &body);

    let Args { user_id } = args.0;
    let Body { array: changes } = body.0;

    let connection: &mut MysqlConnection = &mut data.get_connection();

    handle_request(|| {
        let session = authenticate_request(connection, req)?;
        let permissions = session.get_permissions(user_id);

        if !permissions.access_levels.edit {
            return Err(HttpResponse::Unauthorized().json(UnauthorizedResponse::NoPermission));
        }

        let old_access_levels = db_load_access_levels_by_user_id_and_access_level(
            connection,
            user_id,
            AccessLevel::MAX_LEVEL,
        )
        .internal()?;

        let delete_access_levels = old_access_levels
            .iter()
            .filter(|al| !changes.iter().any(|oal| oal.id == al.id))
            .map(|al| al.id)
            .collect::<Vec<_>>();
        let (new_access_levels, update_access_levels) =
            changes.into_iter().partition::<Vec<_>, _>(|al| al.id == -1);

        let new_access_levels = new_access_levels.into_iter().map(|al| {
            DbNewAccessLevel {
                user_id,
                name: al.name,
                level: al.new_level,
            }
        }).collect::<Vec<_>>();
        
        let update_access_levels = update_access_levels.into_iter().map(|al| {
            DbUpdateAccessLevel {
                id: al.id,
                name: Some(al.name),
                level: Some(al.new_level),
            }
        }).collect::<Vec<_>>();

        db_delete_access_levels_by_ids(connection, &delete_access_levels).internal()?;
        db_insert_access_levels(connection, &new_access_levels).internal()?;

        for upd_access_level in &update_access_levels {
            db_update_access_level(connection, upd_access_level).internal()?;
        }

        Ok(HttpResponse::Ok().json(Response {}))
    })
}
