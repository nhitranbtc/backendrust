use std::pin::Pin;
use crate::utils::frankjwt::{create_token_rs256, validate_signature_jwt_rs256, decode_jwt_rs256, test_encoded_rs256};
use chrono::NaiveDateTime; // This type is used for date field in Diesel.
use std::time::{Duration, Instant};
use actix_identity::Identity;
use actix_http::{http, Request, Response};

use actix_web::{
    dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest,
    HttpResponse,
};
use diesel::prelude::*;
use crate::errors::{MyStoreError, ServiceError};
use crate::models::user::{AuthUser};
use crate::db_connection::PgPool;
use crate::handlers::pg_pool_handler;

pub async fn login(auth_user: web::Json<AuthUser>,
    id: Identity,
    pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse> {
    let pg_pool = pg_pool_handler(pool)?;

    let user = auth_user
        .login(&pg_pool)
        .map_err(|e| {
            match e {
                MyStoreError::DBError(diesel::result::Error::NotFound) =>
                    HttpResponse::NotFound().json(e.to_string()),
                _ =>
                    HttpResponse::InternalServerError().json(e.to_string())
            }
        })?;
    // This is the jwt token we will send in a cookie.
    let token = create_token_rs256(&user.email, &user.company)?;
    println!("token: {:?}", token);
    //id.remember(token);
    //test_identity(id);

    //let cookie_value = TokenGenerator::new(usize::MIN,usize::MAX).as_str();
    let response =
        HttpResponse::Ok()
        .header("ACCESS_TOKEN", token)
        .cookie(
            http::Cookie::build("backendrust", "cookie_value")
                .domain(dotenv!("DOMAIN"))
                .path("/")
                .secure(true)
                .http_only(true)
                .finish(),
        )
        .json(user);
    Ok(response)
}

pub async fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().into())
}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
     let res = validate_signature_jwt_rs256(&token);
     Ok(res.is_ok())
}

#[allow(dead_code)]
fn test_identity(id: Identity) {
    // access request identity
    match id.identity() {
        Some(id) => {
            println!("Welcome! {}", id)
        }
        None=> println!("Anonymous")
    }
}