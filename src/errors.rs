use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

use std::fmt;
use bcrypt::BcryptError;
use diesel::result;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = " Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Interal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}

#[derive(Debug)]
pub enum MyStoreError {
    InternalServerError(result::Error),
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String)
}

// We neea this to performs a conversion from BcryptError to MyStoreError
impl From<BcryptError> for MyStoreError {
    fn from(err: BcryptError) -> Self {
       MyStoreError::HashError(err)
    }
}

// We need this to performs a conversion from diesel::result::Error to MyStoreError
impl From<result::Error> for MyStoreError {
    fn from(error: result::Error) -> Self {
        MyStoreError::DBError(error)
    }
}

impl fmt::Display for MyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyStoreError::InternalServerError(error) => write!(f, "{}", error),
            MyStoreError::HashError(error) => write!(f, "{}", error),
            MyStoreError::DBError(error) => write!(f, "{}", error),
            MyStoreError::PasswordNotMatch(error) => write!(f, "{}", error),
            MyStoreError::WrongPassword(error) => write!(f, "{}", error),
        }
    }
}