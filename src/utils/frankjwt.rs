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
    //exp: u64,

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
            //exp : utc,
            exp: (Local::now() + Duration::seconds(60)).timestamp() as usize,
        }
    }
}
#[allow(dead_code)]
fn test_exp() -> bool {
    use std::time::{SystemTime, UNIX_EPOCH};
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("exp {}", exp -2);
    let utc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    (exp + 0) > utc
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

pub fn validate_signature_jwt_rs256(jwt1: &str) -> Result<bool, ServiceError> {
    let mut path = env::current_dir().unwrap();
    path.push("test");
    path.push("my_rsa_2048_key.pem");
    path.to_str().unwrap().to_string();
    let validate = validate_signature(&jwt1, &get_rsa_256_public_key_full_path(), Algorithm::RS256);
    match validate {
        Ok(b) => Ok(b),
        Err(_e) => Err(ServiceError::JWKSFetchError),
    }
}


pub fn decode_jwt_rs256(token: &str) -> Result<SlimUser, ServiceError> {
    let decoded = decode(
        &token,
        &get_rsa_256_public_key_full_path(),
        Algorithm::RS256,
        &ValidationOptions::default(),
    );
    println!("Decode {:?}", decoded);
    match decoded {
        Ok(v) => {
            let (header, payload) = v;
            //println!("header \n{}", header);
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
pub fn create_token(email: &str, company: &str) -> Result<String, HttpResponse> {
    let claims = json!(Claims::with_email(email, company));
    let secret = dotenv!("JWT_SECRET").to_string();
    let header = json!({});
    encode(header, &secret, &claims, Algorithm::HS256)
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, ServiceError> {
    let secret = dotenv!("JWT_SECRET").to_string();
    let decoded = decode(
        token,
        &secret,
        Algorithm::HS256,
        &ValidationOptions::default(),
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

    #[test]
    fn test_leeway_exp() {
        let utc = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let p1 = json!({
            "exp" : utc - 2,
        });

        let secret = "secret123".to_string();
        let header = json!({});
        let jwt = encode(header, &secret, &p1, Algorithm::HS512).unwrap();

        let mut validation = ValidationOptions::default();
        //validation.exp_leeway = 5;
        let result = decode(&jwt, &String::from("secret123"), Algorithm::HS512, &validation);
        assert_eq!(result.is_ok(), true);
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
