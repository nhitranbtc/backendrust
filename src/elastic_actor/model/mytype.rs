///! https://docs.rs/elastic/0.21.0-pre.5/elastic/types/string/text/struct.Text.html

use elastic::prelude::*;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, ElasticType)]
//#[elastic(index(expr = "self.index()"))]
#[elastic(index = "mytype")]
pub struct IndexDoc {
    #[elastic(id)]
    pub id: Id,
    pub title: String,
    pub generation: i64,
    pub nested: Option<MyNested>,
    pub object: MyObject,
    pub timestamp: Date<DefaultDateMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ElasticType)]
#[elastic(index = "mytype")]
pub struct Doc {
    #[elastic(id)]
    pub id: Id,
    pub title: String,
    pub generation: i64,
    pub nested: Option<Value>,
    pub object: MyObject,
    pub timestamp: Date<DefaultDateMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDoc {
    pub title: String,
    pub generation: i64,
    pub nested: Option<Value>,
    pub object: MyObject
}

pub type Id = Keyword<DefaultKeywordMapping>;

#[derive(Debug, Clone, Serialize, Deserialize, ElasticType)]
pub struct MyNested {
    //pub my_date: Date<DefaultDateMapping>,
    //pub my_string: String,
    pub my_num: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, ElasticType)]
#[elastic(mapping="MyObject")]
pub struct MyObject {
    //pub my_date: Date<DefaultDateMapping>,
    //pub my_string: String,
    pub my_num: i32
}

impl ObjectMapping for MyObject {
    type Properties = MyObject;
    fn data_type() -> &'static str { OBJECT_DATATYPE }
}


impl IndexDoc {
    #[allow(dead_code)]
    fn index() -> Index<'static> {
        "mytype".into()
    }
}

impl Doc {
    pub fn new() -> Self {
        let v = json!({"my_num": 2});
        let data = json!({
            "title":"Title ",
            "generation": 0,
            "nested": json!([v]),
            "object": json!({"my_num": 1}),
        });
        Self::index_res(data)
    }

    pub fn index_res(data: Value) -> Self {
        let mytype: UpdateDoc = serde_json::from_value(data).unwrap();
        let uuid = Uuid::new_v4().to_string();
        Self {
            id: Keyword::<DefaultKeywordMapping>::new(uuid),
            title: mytype.title,
            generation: mytype.generation,
            nested: mytype.nested,
            object: mytype.object,
            timestamp: Date::now(),
        }
    }

}
