#[macro_use]
extern crate diesel;
extern crate actix;
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen; // Using the dotenv! macro
extern crate crypto;
extern crate frank_jwt;
extern crate hyper;

//use actix_web::Responder;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{dev::ServiceRequest, http, post, web, App, Error, HttpResponse, HttpServer};
use dotenv::dotenv;
use std::env;

use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use std::thread;

use console::Term;

use ::mystore_lib::backtrace_example::raw;
use ::mystore_lib::bastion_actor::{
    bastion_nats, bastion_pool, broadcast_message, callbacks, getting_started, middleware,
    parallel, restart_strategy, send_recv,
};
use ::mystore_lib::db_connection::establish_connection;
use ::mystore_lib::elastic_actor::model::*;
use ::mystore_lib::eventsourcing_actor::{account};
use ::mystore_lib::nats_actor::request;
use ::mystore_lib::order_topic::{publish, subscribe};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::{Duration, Local};
use token_generator::TokenGenerator;

#[derive(RustcDecodable, RustcEncodable)]
struct UserLogin {
    email: String,
    password: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //dotenv::dotenv().ok();
    //std::env::set_var("RUST_LOG", "actix_web=debug");
    //env_logger::init();

    use tracing::Level;
    let subscriber = tracing_subscriber::fmt()
        // all spans/events with a level higher than INFO
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder and sets the constructed 'Subscriber' as the default.
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    account::account();

    let term = Term::stdout();
    term.write_line("Hello World!")?;
    thread::sleep_ms(1000);
    term.clear_line()?;
    use console::style;

    println!("This is {} neat", style("quite").cyan());

    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);

    // Start http server
    HttpServer::new(move || {
        let access_token_header = header::HeaderName::from_lowercase(b"access_token").unwrap();
        App::new()
            .wrap(Logger::default())
            // we implement middleares with the warp method
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                    .domain(dotenv!("DOMAIN"))
                    .name("backendrust")
                    .path("/")
                    .max_age(Duration::days(1).num_seconds())
                    .secure(dotenv!("COOKIE_SECURE").parse().unwrap()),
            ))
            .wrap(
                Cors::new()
                    //.allowed_origin(dotenv!("ALLOWED_ORIGIN"))
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                        http::header::ACCEPT,
                        access_token_header.clone(),
                    ])
                    //.expose_headers(vec![access_token_header.clone()])
                    .max_age(3600)
                    .finish(),
            )
            //.data(establish_connection())
            .service(
                web::resource("/users").route(web::get().to(::mystore_lib::handlers::apis::get)),
            )
            .service(
                web::resource("/register")
                    .route(web::post().to(::mystore_lib::handlers::register::register)),
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(::mystore_lib::handlers::authentication::login))
                    .route(web::delete().to(::mystore_lib::handlers::authentication::logout)),
            )
        //.service(
        //    web::resource("/bastion")
        //    .route(web::get().to(::mystore_lib::bastion_actor::parallel::bastion))
        //)
        //.route("/users", web::get().to(handlers::user_api::get_users))
        //.route("/users/{id}", web::get().to(handlers::get_user_by_id))
        //.route("/users", web::post().to(handlers::add_user))
        //.route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(&bind)?
    .run()
    .await
}
