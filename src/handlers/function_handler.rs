macro_rules! function_handler {
    ( $handler_name:ident ($($arg:ident:$typ:ty),*) -> $body:expr) => {
        pub async fn $handler_name(user: LoggedUser, pool: web::Data<Pool>, $($arg:$typ,)*)
            -> Result<HttpResponse, Error>
        {
            // web::block(move || {
            //     // let pg_pool = pool
            //     //     .get()
            //     //     .map_err(|_| {
            //     //         crate::errors::MyStoreError::PGConnectionError
            //     //     })?;

            //     $body(user, pool)
            // })
            // .await
            // .map(|user| HttpResponse::Ok().json(user))
            // .map_err(|error| actix_web::error::ErrorInternalServerError(error))
            // .map(|res| match res {
            //     Ok(data) => Ok(HttpResponse::Ok().json(data)),
            //     Err(error) => Err(actix_web::error::ErrorInternalServerError(error)),
            // })
        Ok(web::block(move || $body(user, pool))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(|_| HttpResponse::InternalServerError())?
        )
        }
    };
}
