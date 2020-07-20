use elastic::client::responses::search::{Documents, Hits};
use elastic::{prelude::*, Error as ResponseError};

use serde_json::Error as JsonError;
use serde_json::Value;

use crate::elastic_actor::elastic_store::Client;
use crate::elastic_actor::model;
use std::io::Error as IoError;

type MyType = model::mytype::MyType;
type Response = GetResponse<MyType>;

pub trait SearchDocument {
    fn search(&self, query: &str) -> Result<SearchResponse<MyType>, ResponseError>;
}

impl SearchDocument for Client {
    fn search(&self, query: &str) -> Result<SearchResponse<MyType>, ResponseError> {
        let response = self
            .io
            .document::<MyType>()
            .search()
            .body(json!({
                "query":{
                    "query_string":{
                        "query":query
                    }
                }
            }))
            .send()?;
        Ok(response)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum IndexDocError {
        Io(err: IoError) {
            from()
                display("failed to index doc: {}", err)
        }
        Json(err: JsonError) {
            from()
                display("failed to index doc: {}", err)

        }
        Response(err: ResponseError) {
            from()
                display("failed to index doc: {}", err)
        }
    }
}
