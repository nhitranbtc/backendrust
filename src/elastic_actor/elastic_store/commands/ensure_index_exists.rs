use elastic::{
    http::StatusCode,
    prelude::*,
    Error as ResponseError,
};

use serde_json::Error as JsonError;
use serde_json::Value;
use std::io::Error as IoError;
use crate::elastic_actor::elastic_store::{Client};
use crate::elastic_actor::{model};

type MyType = model::mytype::MyType;

pub trait EnsureIndexExists {
    fn ensure_index_exists(&self) -> Result<(), EnsureIndexExistsError>;
}

impl EnsureIndexExists for Client {
    fn ensure_index_exists(&self) -> Result<(), EnsureIndexExistsError> {
        let exists = self
            .io
            .request(IndicesExistsRequest::for_index(model::mytype_index::name()))
            .send()?;
        match exists.status() {
            // Success, do nothing
            StatusCode::OK => {
                println!("updated index");
                self.io
                    .document::<model::mytype::MyType>()
                    .put_mapping()
                    .send()?;
            },
            // Not found, create the index
            StatusCode::NOT_FOUND => {
                self.io
                    .index(model::mytype_index::name())
                    .create()
                    .body(model::mytype_index::body().to_string())
                    .send()?;
            },
            _ => {
                exists.into_response::<CommandResponse>()?;
            }
        }
        Ok(())
    }

}

pub trait MappingDoc {
    fn get_mapping(&self) -> Result<Value, EnsureIndexExistsError>;
}

impl MappingDoc for Client {
    fn get_mapping(&self) -> Result<Value, EnsureIndexExistsError> {
        let get_mapping = self
            .io
            .request(IndicesGetMappingRequest::for_index(MyType::static_index()))
            .send()
            .and_then(|res| res.into_response::<Value>());
        let index = get_mapping.unwrap();
        Ok(index)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum EnsureIndexExistsError {
        Io(err: IoError) {
            from()
                display("failed to ensure index exists: {}", err)
        }
        Json(err: JsonError) {
            from()
            display("failed to ensure index exists: {}", err)

        }
        Response(err: ResponseError) {
            from()
            display("failed to ensure index exists: {}", err)
        }
    }
}


