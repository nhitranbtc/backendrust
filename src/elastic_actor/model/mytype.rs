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
    pub timestamp: Date<DefaultDateMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMyType {
    pub title: String,
    pub generation: i64
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

    pub fn index_res(data: Value) -> Self {
        let mytype: NewMyType = serde_json::from_value(data).unwrap();
        Self {
            id: Uuid::new_v4().to_string(),
            title: mytype.title,
            generation: mytype.generation,
            timestamp: Date::now(),
        }
    }
}
