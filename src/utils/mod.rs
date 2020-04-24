pub mod frankjwt;

use serde_json;
use dotenv_codegen; // Using the dotenv! macro
use frank_jwt::{Algorithm, ValidationOptions, encode, decode};
