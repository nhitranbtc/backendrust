use actix::prelude::Future;
use crate::models::user::{User, RegisterUser};
use actix_web::{web, Error, HttpResponse};
use crate::errors::MyStoreError;
use crate::models::user::{Pool, AuthUser};

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

use crate::handlers::LoggedUser;
function_handler!(
    // index (product_search: web::Query<ProductSearch>, pagination: web::Query<ProductPagination>)
    //  -> (|user: LoggedUser, pg_pool: PgPooledConnection| {
    //         let search = &product_search.search;
    //         ProductList::list(user.id, search, pagination.rank, &pg_pool)
    //     })

    get(db: web::Data<Pool>) -> (
        |user: LoggedUser, pool: web::Data<Pool>| {
            User::get_all_users(db)
        }
    )
);

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || User::get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
    )
}
