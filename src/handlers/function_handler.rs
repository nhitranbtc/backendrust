macro_rules! function_handler {
    ( $handler_name:ident ($($arg:ident:$typ:ty),*) -> $body:expr) => {
        pub async fn $handler_name(user: LoggedUser, pool: web::Data<PgPool>, $($arg:$typ,)*)
            -> Result<HttpResponse, Error>
        {
            Ok(web::block(move || {
                let pg_pool = pool
                    .get()
                    .map_err(|_| {
                        crate::errors::MyStoreError::PGConnectionError
                    }).unwrap();
            $body(user, pg_pool)
        })
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?
        )}
    };
}
