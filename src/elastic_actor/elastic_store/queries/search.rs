use elastic::{prelude::*, Error as ResponseError};
use serde_json::Error as JsonError;
use std::io::Error as IoError;

use crate::elastic_actor::elastic_store::Client;
use crate::elastic_actor::model::mytype::Doc;

type Response = GetResponse<Doc>;

pub trait SearchDocument {
    fn search(&self, query: &str) -> Result<SearchResponse<Doc>, ResponseError>;
}

impl SearchDocument for Client {
    fn search(&self, query: &str) -> Result<SearchResponse<Doc>, ResponseError> {
        let response = self
            .io
            .document::<Doc>()
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
