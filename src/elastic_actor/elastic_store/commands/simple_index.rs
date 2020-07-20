use elastic::client::responses::search::{Documents, Hits};
use elastic::{prelude::*, Error as ResponseError};

use serde_json::Error as JsonError;
use serde_json::Value;

use crate::elastic_actor::elastic_store::Client;
use crate::elastic_actor::model;
use std::io::Error as IoError;

type MyType = model::mytype::MyType;

pub trait Document {
    fn save_sample_doc(&self) -> Result<(), IndexDocError>;
    fn index(&self, data: Value) -> Result<(), IndexDocError>;
    fn index_refresh(&self, data: Value) -> Result<(), IndexDocError>;
    fn update(&self, id: String, data: Value) -> Result<(), IndexDocError>;
    fn update_raw(&self, id: String, data: Value) -> Result<(), IndexDocError>;
    fn update_refresh(&self, id: String, data: Value) -> Result<(), IndexDocError>;
    fn get(&self, id: &str) -> Result<MyType, ResponseError>;
}


impl Document for Client {
    fn save_sample_doc(&self) -> Result<(), IndexDocError> {
        let doc: MyType = MyType::create_sample();
        //println!("save {:?}", doc);
        self.io
            .document()
            .index(&doc)
            .params_fluent(|p| p.url_param("refresh", true))
            .send()?;
        //let get_res = self.io.document::<MyType>().get(ID).send();
        //Box::new(
        //    index_res
        //    .and_then(|_| get_res)
        //    );
        Ok(())
    }

    fn index(&self, data: Value) -> Result<(), IndexDocError> {
        let doc: MyType = MyType::index_res(data);
        //println!("save {:?}", doc);
        self.io.document().index(doc).send()?;
        Ok(())
    }

    fn index_refresh(&self, data: Value) -> Result<(), IndexDocError> {
        let doc: MyType = MyType::index_res(data);
        //println!("save {:?}", doc);
        self.io
            .document()
            .index(doc)
            .params_fluent(|p| p.url_param("refresh", true))
            .send()?;
        Ok(())
    }

    fn update(&self, id: String, data: Value) -> Result<(), IndexDocError> {
        let doc: MyType = MyType::index_res(data);
        self.io.document::<MyType>().update(id).doc(doc).send()?;
        Ok(())
    }

    fn update_raw(&self, id: String, data: Value) -> Result<(), IndexDocError> {
        let doc: MyType = MyType::index_res(data);
        let index = MyType::static_index();
        self.io.document().update_raw(index, id).doc(doc).send()?;
        Ok(())
    }

    fn update_refresh(&self, id: String, data: Value) -> Result<(), IndexDocError> {
        let doc: MyType = MyType::index_res(data);
        self.io
            .document::<MyType>()
            .update(id)
            .doc(doc)
            .params_fluent(|p| p.url_param("refresh", true))
            .send()?;
        Ok(())
    }

    fn get(&self, id: &str) -> Result<MyType, ResponseError> {
        let response = self.io.document::<MyType>().get(id.to_owned()).send()?;
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
