use chrono::NaiveDateTime; // This type is used for date field in Diesel.
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use diesel::r2d2::{self, ConnectionManager};

use std::vec::Vec;
use serde::{Deserialize, Serialize};
//use super::Pool;
use actix_web::{web, Error, HttpResponse};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]

pub struct User {
    //#[serde(skip)] // we're removing id from being show in the response
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub company: String,
    #[serde(skip)] // we're removing password from being show in the response()]
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub company: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

use bcrypt::{hash, DEFAULT_COST};
use diesel::PgConnection;
use chrono::Local;
use crate::errors::MyStoreError;

// MyStoreError is a custom error that I will show it next.
impl User {
    pub fn create(register_user: RegisterUser, db: web::Data<Pool>,) -> Result<User, MyStoreError> {
        let connection = db.get().unwrap();
        let new_user = NewUser {
                first_name: register_user.first_name,
                last_name: register_user.last_name,
                email: register_user.email,
                company: register_user.company,
                password: Self::hash_password(register_user.password)?,
                created_at: Local::now().naive_local(),
            };
        Ok(insert_into(users::table)
            .values(&new_user)
            .get_result(&connection)?)
        }
    pub fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
        let conn = pool.get().unwrap();
        let items = users.load::<User>(&conn)?;
        Ok(items)
    }
        // This might look kind of weird,
        // but if something fails it would chain
        // to our MyStorageError Error,
        // otherwise it will gives use the hash,
        // we still need to return a result
        // so we wrap it in an Ok variant from the Result type.
        pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
            Ok(hash(plain, DEFAULT_COST)?)
        }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterUser {
    pub fn validates(self) -> Result<RegisterUser, MyStoreError> {
        if self.password == self.password_confirmation {
            Ok(self)
        } else {
            Err(
                MyStoreError::PasswordNotMatch(
                    "Password and Password Confirmation does not match".to_string()
                )
            )
        }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {
    // The good thing about ? syntax and have a custom error is
    // that the code would look very straightforward, I mean,
    // the other way would imply a lot of pattern matching
    // making it look ugly.
    pub fn login(&self, conn: &PgConnection ) -> Result<User, MyStoreError> {
        use bcrypt::verify;
        //use diesel::QueryDsl;
        //use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        //use crate::schema::users::dsl::email;
        //let conn: &PgConnection = &pool.get().unwrap();
        let mut records =
            users
                .filter(email.eq(&self.email))
                .load::<User>(conn)?;
        let user =
            records
                .pop()
                .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;
        let verify_password =
        verify(&self.password, &user.password)
            .map_err(|_error| {
                MyStoreError::WrongPassword(
                    "Wrong password, check again please".to_string()
                )
            })?;
        if verify_password {
            Ok(user)
        } else {
            Err(MyStoreError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }

    }
}