use crate::errors::ServiceError;
use actix_web::HttpResponse;
use chrono::{Duration, Local};
use frank_jwt::{decode, encode, Algorithm, ValidationOptions};
use serde::{Deserialize, Serialize};

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


#[allow(dead_code)]
pub fn test_decode(token: &str) {
    //println!("token {:?}", token);
	let test = decode_token(token);
	println!("decode {:?}", test);
}

