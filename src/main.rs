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
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use chrono::{Local, Duration};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;


mod errors;
mod handlers;
mod models;
mod schema;
mod utils;
//mod auth;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(RustcDecodable, RustcEncodable)]
struct UserLogin {
    email: String,
    password: String
}

/// Inserts new user with name defined in form.
// #[post("/user")]
// pub async fn add_user() -> Result<HttpResponse, Error>  {
//     let user = models::user::User{
//         id : 1,
//         first_name : "Nhi".to_string(),
//         last_name :"Tran".to_string(),
//         email :"nhitran@gmail.com".to_string(),
//         created_at: chrono::Local::now().naive_local(),
//     };
//     Ok(HttpResponse::Ok().json(user))

// }



// async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     println!("credentials {:?}", credentials);
//     let config = req
//         .app_data::<Config>()
//         .map(|data| data.get_ref().clone())
//         .unwrap_or_else(Default::default);
//     match auth::validate_token(credentials.token()) {
//         Ok(res) => {
//             if res == true {
//                 Ok(req)
//             } else {
//                 Err(AuthenticationError::from(config).into())
//             }
//         }
//         Err(_) => Err(AuthenticationError::from(config).into()),
//     }
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");


    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let bind = "127.0.0.1:8080";

    println!("Starting server at: {}", &bind);

    // Start http server
    HttpServer::new(move || {
        //let auth = HttpAuthentication::bearer(validator);
        App::new()
            //.wrap(auth)
            .wrap(Logger::default())
            // we implement middleares with the warp method
            .wrap(
                IdentityService::new(
                    CookieIdentityPolicy::new(dotenv!("SECRET_KEY").as_bytes())
                        .domain(dotenv!("MYSTOREDOMAIN"))
                        .name("jwt")
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
                                          http::header::ACCEPT])
                    .max_age(3600)
                    .finish()
            )
            .data(pool.clone())
            .service(
                web::resource("/register")
                    .route(web::post().to(handlers::user_api::register))
            )
            .service(
                web::resource("/auth")
                    .route(web::post().to(handlers::authentication::login))
                    .route(web::delete().to(handlers::authentication::logout))
            )

            .route("/users", web::get().to(handlers::user_api::get_users))
            //.route("/users/{id}", web::get().to(handlers::get_user_by_id))
            // .route("/users", web::post().to(handlers::add_user))
            // .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(&bind)?
    .run()
    .await
}

