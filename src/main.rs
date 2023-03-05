use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use api::{auth::*, event_templates::*, events::*, roles::*, schedules::*, user_roles::*};
use calendar_lib::api::*;
use db::connection::establish_pooled_connection;
use serde_json::json;
use state::*;

mod api;
mod db;
mod error;
mod state;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[post("/echo_struct")]
async fn echo_struct(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(json!({ "echo": req_body }).to_string())
}

#[get("/")]
async fn home() -> impl Responder {
    actix_files::NamedFile::open_async("./assets/index.html").await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    log::info!("Startup");

    // Get the port number to listen on.
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse()
        .expect("PORT must be a number");

    let data = web::Data::new(AppState::new(establish_pooled_connection()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8081")
            .allowed_origin("http://localhost:8081")
            .allowed_origin("http://aspid.xyz")
            .allow_any_header()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"]);
        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .app_data(web::Data::new(WorkerState::new()))
            .service(
                web::scope("/api")
                    .service(echo)
                    .service(echo_struct)
                    .service(
                        web::scope("/auth")
                            .route(
                                "/logout",
                                web::method(auth::logout::METHOD.clone()).to(logout_handler),
                            )
                            .route(
                                "/login",
                                web::method(auth::login::METHOD.clone()).to(login_handler),
                            )
                            .route(
                                "/register",
                                web::method(auth::register::METHOD.clone()).to(register_handler),
                            )
                            .route(
                                "/new_password",
                                web::method(auth::new_password::METHOD.clone())
                                    .to(insert_password_handler),
                            ),
                    )
                    // EVENTS
                    .route(
                        "/event",
                        web::method(events::load::METHOD.clone()).to(load_event_handler),
                    )
                    .route(
                        "/events",
                        web::method(events::load_array::METHOD.clone()).to(load_events_handler),
                    )
                    .route(
                        "/event",
                        web::method(events::insert::METHOD.clone()).to(insert_event_handler),
                    )
                    .route(
                        "/event",
                        web::method(events::update::METHOD.clone()).to(update_event_handler),
                    )
                    .route(
                        "/event",
                        web::method(events::delete::METHOD.clone()).to(delete_event_handler),
                    )
                    // ROLES
                    .route(
                        "/roles",
                        web::method(roles::load_array::METHOD.clone()).to(load_roles_handler),
                    )
                    // USER ROLES
                    .route(
                        "/user_roles",
                        web::method(user_roles::load_array::METHOD.clone())
                            .to(load_user_roles_handler),
                    )
                    .route(
                        "/user_role",
                        web::method(user_roles::insert::METHOD.clone())
                            .to(insert_user_role_handler),
                    )
                    .route(
                        "/user_role",
                        web::method(user_roles::delete::METHOD.clone())
                            .to(delete_user_role_handler),
                    )
                    // EVENT TEMPLATES
                    .route(
                        "/event_templates",
                        web::method(event_templates::load_array::METHOD.clone())
                            .to(load_event_templates_handler),
                    )
                    .route(
                        "/event_template",
                        web::method(event_templates::insert::METHOD.clone())
                            .to(insert_event_template_handler),
                    )
                    .route(
                        "/event_template",
                        web::method(event_templates::delete::METHOD.clone())
                            .to(delete_event_template_handler),
                    )
                    // SCHEDULES
                    .route(
                        "/schedules",
                        web::method(schedules::load_array::METHOD.clone())
                            .to(load_schedules_handler),
                    )
                    .route(
                        "/schedule",
                        web::method(schedules::insert::METHOD.clone()).to(insert_schedule_handler),
                    )
                    .route(
                        "/schedule",
                        web::method(schedules::delete::METHOD.clone()).to(delete_schedule_handler),
                    ),
            )
            .service(home)
            .service(actix_files::Files::new("/", "./assets").show_files_listing())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
