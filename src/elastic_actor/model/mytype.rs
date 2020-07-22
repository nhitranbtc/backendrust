use elastic::prelude::*;
use uuid::Uuid;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, ElasticType)]
//#[elastic(index(expr = "self.index()"))]
#[elastic(index = "mytype")]
pub struct MyType {
    #[elastic(id)]
    pub id: String,
    pub title: String,
    pub generation: i64,
    pub nested: MyNested,
    pub object: MyObject,
    pub timestamp: Date<DefaultDateMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMyType {
    pub title: String,
    pub generation: i64,
    pub nested: MyNested,
    pub object: MyObject
}

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


impl MyType {
    #[allow(dead_code)]
    fn index() -> Index<'static> {
        "mytype".into()
    }
    pub fn create_sample() -> Self {
        let data = json!({
            "title":"Title "
        });
        let mytype = Self::index_res(data);
        mytype
    }

    pub fn new() -> Self {
        let data = json!({
            "title":"Title ",
            "generation": 0,
            "nested": json!({"my_num": 1}),
            "object": json!({"my_num": 1})
        });
        let mytype = Self::index_res(data);
        mytype
    }

    pub fn index_res(data: Value) -> Self {
        let mytype: NewMyType = serde_json::from_value(data).unwrap();
        Self {
            id: Uuid::new_v4().to_string(),
            title: mytype.title,
            generation: mytype.generation,
            nested: mytype.nested,
            object: mytype.object,
            timestamp: Date::now(),
        }
    }
}
