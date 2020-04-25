use std::pin::Pin;
use crate::utils::frankjwt::{create_token, decode_token, test_decode};
use chrono::NaiveDateTime; // This type is used for date field in Diesel.
use std::time::{Duration, Instant};
use actix_identity::Identity;
use actix_web::{
    dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest,
    HttpResponse,
};
use diesel::prelude::*;
use futures::future::Future;
use serde::Deserialize;
use crate::errors::MyStoreError;
use crate::models::user::{Pool, AuthUser};


pub async fn login(auth_user: web::Json<AuthUser>,
    id: Identity,
    pool: web::Data<Pool>) -> Result<HttpResponse, HttpResponse> {
    let conn: &PgConnection = &pool.get().unwrap();
    let user = auth_user
        .login(conn)
        .map_err(|e| {
            match e {
                MyStoreError::DBError(diesel::result::Error::NotFound) =>
                    HttpResponse::NotFound().json(e.to_string()),
                _ =>
                    HttpResponse::InternalServerError().json(e.to_string())
            }
        })?;
    // This is the jwt token we will send in a cookie.
    let token = create_token(&user.email, &user.company)?;
    //println!("token: {:?}", token);
    let decode = decode_token(&token);
    println!("decode: {:?}", decode.unwrap());

    id.remember(token);
    let response =
        HttpResponse::Ok()
        //.header("X-CSRF-TOKEN", hex::encode(generator.generate()))
        .json(user);
    Ok(response)
}

pub async fn logout(id: Identity) -> Result<HttpResponse, HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().into())
}
