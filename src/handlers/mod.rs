pub mod user_api;
pub mod authentication;
pub mod async_auth;

use super::Pool;
use actix_web::{FromRequest, HttpRequest, HttpResponse, dev};
use futures::future::{ok, err, Ready};

use actix_identity::Identity;
//use crate::utils::jwt::{decode_token, SlimUser};
//pub type LoggedUser = SlimUser;

//use hex;
// Because I'm using function a lot,
// I'm including it in the mod file accessible to all handlers.