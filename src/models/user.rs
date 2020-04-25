use chrono::{Local,NaiveDateTime}; // This type is used for date field in Diesel.
use crate::schema::users;
use crate::schema::users::dsl::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use diesel::dsl::{delete, insert_into};
use diesel::r2d2::{self, ConnectionManager};
use std::time::{Duration, Instant};


use diesel::PgConnection;
use crate::errors::MyStoreError;

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


// MyStoreError is a custom error that I will show it next.
impl User {
    pub fn create(register_user: RegisterUser, db: web::Data<Pool>,) -> Result<User, MyStoreError> {
        let connection = db.get().unwrap();
        let new_user = NewUser {
                first_name: register_user.first_name,
                last_name: register_user.last_name,
                email: register_user.email,
                company: register_user.company,
                //password: Self::hash_password(register_user.password)?,
                password: Self::hash_password(register_user.password.as_bytes()),
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
        pub fn hash_password_bcrypt(plain: String) -> Result<String, MyStoreError> {
            use bcrypt::{hash, DEFAULT_COST};
            Ok(hash(plain, DEFAULT_COST)?)
        }
        pub fn hash_password(pwd: &[u8]) -> String {
            use rand::Rng;
            use argon2::{self, Config, hash_encoded};
            let salt = rand::thread_rng().gen::<[u8; 32]>();
            let config = Config::default();
            hash_encoded(pwd, &salt, &config).unwrap()
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
    pub fn verify(hash: &str, pwd: &[u8]) -> bool {
        use argon2::{self, Config, verify_encoded};
            verify_encoded(hash, pwd).unwrap_or(false)
    }
    // The good thing about ? syntax and have a custom error is
    // that the code would look very straightforward, I mean,
    // the other way would imply a lot of pattern matching
    // making it look ugly.
    pub fn login(&self, conn: &PgConnection ) -> Result<User, MyStoreError> {
        //use bcrypt::verify;
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
        //let verify_password =
        // verify(&self.password, &user.password)
        //     .map_err(|_error| {
        //         MyStoreError::WrongPassword(
        //             "Wrong password, check again please".to_string()
        //         )
        //     })?;
        let verify_password =
        Self::verify(&user.password, &self.password.as_bytes());
        if verify_password {
            Ok(user)
        } else {
            Err(MyStoreError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }

    }
}
#[test]
fn test_hash() {
    let hashed = User::hash_password_bcrypt("123456789".to_string());
    println!("hashed {:?}", hashed);
    use argon2::{self, Config};
    let start = Instant::now();

    let pwd = b"password";
    let salt = b"somesalt";
    let config = Config::default();
    let encoded = argon2::hash_encoded(pwd, salt, &config);
    println!("encoded {:?}", encoded);
    let duration = start.elapsed();
    println!("Time elapsed in test_hash() is: {:?}", duration);
}