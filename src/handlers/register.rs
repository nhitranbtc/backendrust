use crate::db_connection::PgPool;
use crate::db_connection::PgPooledConnection;
use crate::models::user::{AuthUser, User, RegisterUser};
use actix_web::{web, Error, HttpResponse};
use crate::handlers::pg_pool_handler;

//use crate::errors::MyStoreError;

// We get a new connection pool, validate the data,
// 'password' and 'password_confirmation' should be the same,
// finally we create the user and return it.

pub async fn register(new_user: web::Json<RegisterUser>, pool: web::Data<PgPool>) ->
 Result<HttpResponse, Error> {
    let pg_pool = pg_pool_handler(pool)?;
    let register_user = new_user
        .into_inner()
        .validates()
        .map_err(|e| {
          HttpResponse::InternalServerError().json(e.to_string())})?;

        Ok(web::block(move || User::create(register_user, &pg_pool))
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
    get() -> (
        |user: LoggedUser, pg_pool: PgPooledConnection| {
            User::get_all_users(&pg_pool)
        }
    )
);

