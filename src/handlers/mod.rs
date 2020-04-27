#[macro_use]
pub mod function_handler;
pub mod async_auth;
pub mod authentication;
pub mod user_api;

use crate::errors::ServiceError;
use crate::utils::frankjwt::{
    decode_jwt_rs256, test_encoded_rs256, validate_signature_jwt_rs256, SlimUser,
};
use actix_identity::Identity;
use actix_web::error::ErrorBadRequest;
use actix_web::HttpResponse;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use actix_web::{web, Result};
use futures::future::{err, ok, Ready};

use rand;
use serde::{Deserialize, Serialize};
use token_generator::TokenGenerator;
// Because I'm using function a lot,
// I'm including it in the mod file accessible to all handlers.
pub type LoggedUser = SlimUser;

//use hex;
//use csrf_token::CsrfTokenGenerator;

#[derive(Debug, Deserialize)]
struct Thing {
    name: String,
}

impl FromRequest for Thing {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut dev::Payload) -> Self::Future {
        if rand::random() {
            ok(Thing {
                name: "thing".into(),
            })
        } else {
            err(ErrorBadRequest("no luck"))
        }
    }
}

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
        println!("Access Token {:?}", access_token);

        let token = access_token.to_str().unwrap();
        let valiate = validate_signature_jwt_rs256(&token).unwrap();

        //let decoded_token = decode_jwt_rs256(token).unwrap();
        if valiate {
            let user: SlimUser = decode_jwt_rs256(&token).unwrap();
            ok(Some(user).unwrap())

        } else {
            err(HttpResponse::Unauthorized().json("Unauthorized".to_string()))
        }
    }
}

fn index(id: Identity) -> String {
    // access request identity
    if let Some(id) = id.identity() {
        format!("Welcome! {}", id)
    } else {
        "Welcome Anonymous!".to_owned()
    }
}

