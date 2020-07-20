#[macro_use]
pub mod function_handler;
pub mod authentication;
pub mod register;
pub mod apis;

use crate::utils::frankjwt::{
    decode_jwt_rs256, validate_signature_jwt_rs256, SlimUser,
};
use crate::db_connection::{ PgPool, PgPooledConnection };

use actix_identity::Identity;
use actix_web::{dev, Error, FromRequest, HttpRequest, HttpResponse};
use actix_web::{web, Result};

use futures::future::{err, ok, Ready};
use serde::{Deserialize, Serialize};

//use crate::errors::ServiceError;
//use rand;
//use token_generator::TokenGenerator;

// Because I'm using function a lot,
// I'm including it in the mod file accessible to all handlers.

pub fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection> {
    pool
    .get()
    .map_err(|e| {
        actix_web::error::ErrorInternalServerError(e)
    })
}

pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = HttpResponse;
    type Future = Ready<Result<Self, HttpResponse>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        let access_token =
            req
                .headers()
                .get("access_token")
                .ok_or(HttpResponse::Unauthorized()).unwrap();
        //println!("Access Token {:?}", access_token);

        let token = access_token.to_str().unwrap();
        let valiate = validate_signature_jwt_rs256(&token).unwrap();

        if valiate {
            match decode_jwt_rs256(&token) {
                Ok(user) => {
                    ok(Some(user).unwrap())
                }
                Err(_) => err(HttpResponse::Unauthorized().json("Unauthorized".to_string()))
            }

        } else {
            err(HttpResponse::Unauthorized().json("Unauthorized".to_string()))
        }
    }
}


