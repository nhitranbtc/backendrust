use crate::errors::ServiceError;
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let domain = std::env::var("DOMAIN").expect("DOMAIN must be set");
    let jwks = fetch_jwks(&format!("{}{}", domain.as_str(), ".well-known/jwks.json"))
        .expect("failed to fetch jwks");
    let validations = vec![Validation::Issuer(domain), Validation::SubjectPresent];
    let kid = match token_kid(&token) {
        Ok(res) => res.expect("failed to decode kid"),
        Err(_) => return Err(ServiceError::JWKSFetchError),
    };
    let jwk = jwks.find(&kid).expect("Specified key not found in set");
    let res = validate(token, jwk, validations);
    Ok(res.is_ok())
}

fn fetch_jwks(uri: &str) -> Result<JWKS, Box<dyn Error>> {
    let mut res = reqwest::get(uri)?;
    let val = res.json::<JWKS>()?;
    return Ok(val);
}

#[test]
fn test_token_kid() {
    let jwt = "eyJraWQiOiI4ckRxOFB3MEZaY2FvWFdURVZRbzcrVGYyWXpTTDFmQnhOS1BDZWJhYWk0PSIsImFsZyI6IlJTMjU2IiwidHlwIjoiSldUIn0.eyJpc3MiOiJhdXRoLnRlc3QuYXByaWxhLm5vIiwiaWF0IjoxNTM2MDUwNjkzLCJleHAiOjE1MzYwNTQyOTMsInN1YiI6IjQyIiwiZXh0Ijoic21va2V0ZXN0IiwicHJ2IjoiYXJpc3RpIiwic2NwIjoicHJvY2VzcyJ9.gOLsv98109qLkmRK6Dn7WWRHLW7o8W78WZcWvFZoxPLzVO0qvRXXRLYc9h5chpfvcWreLZ4f1cOdvxv31_qnCRSQQPOeQ7r7hj_sPEDzhKjk-q2aoNHaGGJg1vabI--9EFkFsGQfoS7UbMMssS44dgR68XEnKtjn0Vys-Vzbvz_CBSCH6yQhRLik2SU2jR2L7BoFvh4LGZ6EKoQWzm8Z-CHXLGLUs4Hp5aPhF46dGzgAzwlPFW4t9G4DciX1uB4vv1XnfTc5wqJch6ltjKMde1GZwLR757a8dJSBcmGWze3UNE2YH_VLD7NCwH2kkqr3gh8rn7lWKG4AUIYPxsw9CB";

    let kid = token_kid(&jwt).expect("Failed to extract token KID");
    assert_eq!(
        Some("8rDq8Pw0FZcaoXWTEVQo7+Tf2YzSL1fBxNKPCebaai4=".into()),
        kid,
        "Extracted KID did not match expected KID"
    );
}
