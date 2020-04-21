use crate::models::user::{User, RegisterUser};
use super::Pool;
use actix_web::{web, Error, HttpResponse};
use crate::errors::MyStoreError;

// We get a new connection pool, validate the data,
// 'password' and 'password_confirmation' should be the same,
// finally we create the user and return it.

pub async fn register(new_user: web::Json<RegisterUser>, pool: web::Data<Pool>) ->
 Result<HttpResponse, Error> {
     let register_user = new_user
        .into_inner()
        .validates()
        .map_err(|e| {
          HttpResponse::InternalServerError().json(e.to_string())})?;

        Ok(web::block(move || User::create(register_user, pool))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
        )
}


pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || User::get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}
