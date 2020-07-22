use elastic::prelude::*;
use serde_json::Value;
use super::mytype::IndexDoc;

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
        "mappings": IndexDoc::index_mapping()
            //Account::static_ty(): Account::index_mapping()
    })
}



