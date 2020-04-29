use crate::db_connection::PgPool;
use crate::db_connection::PgPooledConnection;
use crate::models::user::{User};
use actix_web::{web, Error, HttpResponse};

use crate::handlers::LoggedUser;

function_handler!(
    get() -> (
        |user: LoggedUser, pg_pool: PgPooledConnection| {
            User::get_all_users(&pg_pool)
        }
    )
);

