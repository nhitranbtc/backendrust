use crate::errors::ServiceError;
use actix_web::HttpResponse;
use chrono::{Duration, Local};
use frank_jwt::{decode, encode, validate_signature, Algorithm, ValidationOptions};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

// We're using a struct so we can implement a conversion from
// Claims to SlimUser, useful in the decode function.
#[derive(Debug)]
pub struct SlimUser {
    pub email: String,
    pub company: String,
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.sub,
            company: claims.company,
        }
    }
}

impl Claims {
    fn with_email(email: &str, company: &str) -> Self {
        Claims {
            sub: email.into(),
            company: company.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}

#[allow(dead_code)]
pub fn create_token(email: &str, company: &str) -> Result<String, HttpResponse> {
    let claims = json!(Claims::with_email(email, company));
    let secret = dotenv!("JWT_SECRET").to_string();
    let header = json!({});
    encode(header, &secret, &claims, Algorithm::HS256)
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn create_token_rs256(email: &str, company: &str) -> Result<String, HttpResponse> {
    let claims = json!(Claims::with_email(email, company));
    let header = json!({});
    let mut path = env::current_dir().unwrap();
    path.push("test");
    path.push("my_rsa_2048_key.pem");
    path.to_str().unwrap().to_string();
    encode(
        header,
        &get_rsa_256_private_key_full_path(),
        &claims,
        Algorithm::RS256,
    )
    .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn validate_signature_jwt_rs256(jwt1: &str) {
    let mut path = env::current_dir().unwrap();
    path.push("test");
    path.push("my_rsa_2048_key.pem");
    path.to_str().unwrap().to_string();
    let maybe_res =
        validate_signature(&jwt1, &get_rsa_256_public_key_full_path(), Algorithm::RS256);
    println!("maybe_res {:?}", maybe_res);
    //assert!(maybe_res.unwrap());
}

pub fn decode_token(token: &str) -> Result<SlimUser, ServiceError> {
    let secret = dotenv!("JWT_SECRET").to_string();
    let decoded = decode(
        token,
        &secret,
        Algorithm::HS256,
        &ValidationOptions::dangerous(),
    );
    match decoded {
        Ok(v) => {
            let (_header, payload) = v;
            //println!("header {}", header);
            //println!("payload {}", payload);
            let email = payload["sub"].as_str().unwrap();
            let company = payload["company"].as_str().unwrap();
            let claims = Claims::with_email(email, company);
            //println!("claims {:?}", claims);
            let new_user = SlimUser::from(claims);
            Ok(new_user)
        }
        Err(_e) => Err(ServiceError::Unauthorized),
    }
}

pub fn decode_jwt_rs256(token: &str) -> Result<SlimUser, ServiceError> {
    let decoded = decode(
        &token,
        &get_rsa_256_public_key_full_path(),
        Algorithm::RS256,
        &ValidationOptions::dangerous(),
    );
    match decoded {
        Ok(v) => {
            let (header, payload) = v;
            println!("header \n{}", header);
            //println!("payload {}", payload);
            let email = payload["sub"].as_str().unwrap();
            let company = payload["company"].as_str().unwrap();
            let claims = Claims::with_email(email, company);
            //println!("claims {:?}", claims);
            let new_user = SlimUser::from(claims);
            Ok(new_user)
        }
        Err(_e) => Err(ServiceError::Unauthorized),
    }
}

fn get_rsa_256_public_key_full_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("test");
    path.push("my_rsa_public_2048_key.pem");
    path.to_path_buf()
}

fn get_rsa_256_private_key_full_path() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push("test");
    path.push("my_rsa_2048_key.pem");
    path.to_path_buf()
}

#[allow(dead_code)]
pub fn test_decode(token: &str) {
    //println!("token {:?}", token);
    let test = decode_token(token);
    println!("decode {:?}", test);
}

#[allow(dead_code)]
pub fn test_encoded_rs256() {
    let p1 = json!({
        "key1" : "val1",
        "key2" : "val2",
        "key3" : "val3"
    });

    let header = json!({});
    let mut path = env::current_dir().unwrap();
    path.push("test");
    path.push("my_rsa_2048_key.pem");
    path.to_str().unwrap().to_string();

    let jwt1 = encode(
        header,
        &get_rsa_256_private_key_full_path(),
        &p1,
        Algorithm::RS256,
    )
    .unwrap();
    println!("jwt1 {}", jwt1);
}
