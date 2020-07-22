use elastic::client::responses::search::{Documents, Hits};
use elastic::{prelude::*, Error as ResponseError};

use serde_json::Error as JsonError;
use serde_json::Value;

use crate::elastic_actor::elastic_store::Client;
use crate::elastic_actor::model;
use std::io::Error as IoError;

type Doc = model::mytype::Doc;

pub trait Document {
    fn new(&self) -> Result<(), IndexDocError>;
    fn index(&self, data: Value) -> Result<(), IndexDocError>;
    fn index_refresh(&self, data: Value) -> Result<(), IndexDocError>;
    fn update(&self, id: String, data: Value) -> Result<(), IndexDocError>;
    fn update_raw(&self, id: String, data: Value) -> Result<(), IndexDocError>;
    fn update_refresh(&self, id: String, data: Value) -> Result<(), IndexDocError>;
    fn get(&self, id: &str) -> Result<Doc, ResponseError>;
}

impl Document for Client {
    fn new(&self) -> Result<(), IndexDocError> {
        let doc: Doc = Doc::new();
        println!("save {:?}", doc);
        self.io
            .document()
            .index(&doc)
            .params_fluent(|p| p.url_param("refresh", true))
            .send()?;
        Ok(())
    }

    fn index(&self, data: Value) -> Result<(), IndexDocError> {
        let doc: Doc = Doc::index_res(data);
        //println!("save {:?}", doc);
        self.io.document().index(doc).send()?;
        Ok(())
    }

    fn index_refresh(&self, data: Value) -> Result<(), IndexDocError> {
        let doc: Doc = Doc::index_res(data);
        //println!("save {:?}", doc);
        self.io
            .document()
            .index(doc)
            .params_fluent(|p| p.url_param("refresh", true))
            .send()?;
        Ok(())
    }

    fn update(&self, id: String, data: Value) -> Result<(), IndexDocError> {
        let doc: Doc = Doc::index_res(data);
        self.io.document::<Doc>().update(id).doc(doc).send()?;
        Ok(())
    }

    fn update_raw(&self, id: String, data: Value) -> Result<(), IndexDocError> {
        let doc: Doc = Doc::index_res(data);
        let index = Doc::static_index();
        self.io.document().update_raw(index, id).doc(doc).send()?;
        Ok(())
    }

    fn update_refresh(&self, id: String, data: Value) -> Result<(), IndexDocError> {
        let doc: Doc = Doc::index_res(data);
        self.io
            .document::<Doc>()
            .update(id)
            .doc(doc)
            .params_fluent(|p| p.url_param("refresh", true))
            .send()?;
        Ok(())
    }

    fn get(&self, id: &str) -> Result<Doc, ResponseError> {
        let response = self.io.document::<Doc>().get(id.to_owned()).send()?;
        let doc = response.into_document().unwrap();
        Ok(doc)
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
