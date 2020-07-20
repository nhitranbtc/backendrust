use super::mytype::MyType;
use elastic::prelude::*;
use serde_json::Value;

/// Get the name of the index.
pub fn name() -> Index<'static> {
    "mytype".into()
}



/// Get the setting and mappings for the index
pub fn body() -> Value {
    json!({
        "settings": {
            "analysis" : {
                "filter" : {
                    "email": {
                        "type": "pattern_capture",
                        "preserve_original":true,
                        "patterns": [
                            "([^@]+)",
                            "(\\p{L}+)",
                            "(\\d+)",
                            "@(.+)"
                        ]
                    }
                },
                "analyzer": {
                    "email": {
                        "tokenizer": "uax_url_email",
                        "filter": [
                            "email",
                            "lowercase",
                            "unique"
                        ]
                    }
                }
            }
        },
        "mappings": MyType::index_mapping()
            //Account::static_ty(): Account::index_mapping()
    })
}



