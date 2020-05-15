#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
//#[macro_use]
//extern crate diesel_derive_enum;

extern crate actix;
extern crate actix_cors;
extern crate actix_identity;
extern crate actix_web;

//extern crate argon2;
extern crate bcrypt;
extern crate frank_jwt;
extern crate jsonwebtoken as jwt;
extern crate token_generator;

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate actix_http;
//extern crate diesel_full_text_search;

extern crate tokio;

#[macro_use]
extern crate juniper;
extern crate juniper_subscriptions;
extern crate juniper_warp;

// Implement NATs
extern crate nats;

pub mod db_connection;
pub mod errors;
//pub mod graphql;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod utils;
pub mod nats_actor;
