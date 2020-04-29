#[macro_use]
extern crate diesel;
extern crate log;
extern crate actix;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen; // Using the dotenv! macro
extern crate hyper;
extern crate crypto;
extern crate frank_jwt;
//use actix_web::Responder;

use dotenv::dotenv;
use std::env;
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, post, http, web, App, Error, HttpServer, HttpResponse};
use actix_web::http::header;

use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use chrono::{Local, Duration};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use token_generator::TokenGenerator;
use ::mystore_lib::db_connection::establish_connection;


#[derive(RustcDecodable, RustcEncodable)]
struct UserLogin {
    email: String,
    password: String
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);

    // Start http server
    HttpServer::new(move || {
        let access_token_header = header::HeaderName::from_lowercase(b"access_token").unwrap();
        App::new()
            .wrap(Logger::default())
            // we implement middleares with the warp method
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                        .domain(dotenv!("DOMAIN"))
                        .name("backendrust")
                        .path("/")
                        .max_age(Duration::days(1).num_seconds())
                        .secure(dotenv!("COOKIE_SECURE").parse().unwrap())
                )
            )
            .wrap(
                Cors::new()
                    //.allowed_origin(dotenv!("ALLOWED_ORIGIN"))
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION,
                                          http::header::CONTENT_TYPE,
                                          http::header::ACCEPT,
                                          access_token_header.clone()])
                    //.expose_headers(vec![access_token_header.clone()])
                    .max_age(3600)
                    .finish()
            )
            .data(establish_connection())
            .service(
            web::resource("/users")
                .route(web::get().to(::mystore_lib::handlers::apis::get))
            )
            .service(
                web::resource("/register")
                    .route(web::post().to(::mystore_lib::handlers::register::register))
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(::mystore_lib::handlers::authentication::login))
                    .route(web::delete().to(::mystore_lib::handlers::authentication::logout))
            )
            //.route("/users", web::get().to(handlers::user_api::get_users))
            //.route("/users/{id}", web::get().to(handlers::get_user_by_id))
            //.route("/users", web::post().to(handlers::add_user))
            //.route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(&bind)?
    .run()
    .await
}

