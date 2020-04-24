use std::pin::Pin;

use actix_identity::Identity;
use actix_web::{
    dev::Payload, error::BlockingError, web, Error, FromRequest, HttpRequest,
    HttpResponse,
};
use diesel::prelude::*;
use futures::future::Future;
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::user::{Pool, AuthUser, User};

// We get a new connection pool, then look up for the user,
// If there is no user a NotFound error would raise otherwise
// this would just through an InternalServerError.

pub async fn login(auth_user: web::Json<AuthUser>,
    id: Identity,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(auth_user.into_inner(), pool)).await;
    match res {
        Ok(user) => {
            let user_string = serde_json::to_string(&user).unwrap();
            id.remember(user_string);
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

/// Diesel query
fn query(auth_data: AuthUser, pool: web::Data<Pool>) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    use bcrypt::verify;
    let conn = &pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;
    //println!("pass {:?}", &auth_data.password);
    // let verify_password =
    //     verify(&auth_data.password, &user.password)
    //         .map_err(|_error| {
    //             MyStoreError::WrongPassword(
    //                 "Wrong password, check again please".to_string()
    //             )
    //         })?;
    // if verify_password {
    //     OK(user)
    // } else {
    //     Err(MyStoreError::WrongPassword(
    //         "Wrong password, check again please".to_string
    //     ))
    // }

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&auth_data.password, &user.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

