use chrono::{Local, Duration};
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use frank_jwt::{Algorithm, ValidationOptions, encode, decode};
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

// We're using a struct so we can implement a conversion from
// Claims to SlimUser, useful in the decode function.
pub struct SlimUser {
    pub email: String,
    pub company: String
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.sub,
            company: claims.company
        }
    }
}

impl Claims {
    fn with_email(email: &str, company: &str) -> Self {
        Claims {
            sub: email.into(),
            company: company.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize
        }
    }
}

pub fn create_token(email: &str, company: &str) -> Result<String, HttpResponse> {
    let claims = json!(Claims::with_email(email, company));
    //let secret = "secret123".to_string();
    let secret = dotenv!("JWT_SECRET").to_string();
    let header = json!({});
    encode( header,&secret, &claims, Algorithm::HS384)
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}
