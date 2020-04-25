extern crate frank_jwt;
#[macro_use]
extern crate serde_json;


use frank_jwt::{Algorithm, ValidationOptions, encode, decode};

   #[test]
    fn test_encode_and_decode_jwt_hs384() {
        let p1 = json!({
            "key1" : "val1",
            "key2" : "val2",
            "key3" : "val3"
        });
        //use std::time::{Duration, Instant};
        //let start = Instant::now();
        //let duration = start.elapsed();
        //println!("Time elapsed in expensive_function() is: {:?}", duration);

        let secret = "secret123".to_string();
        let  header = json!({});
        let jwt1 = encode(header, &secret, &p1, Algorithm::HS384).unwrap();
        let maybe_res = decode(&jwt1, &secret, Algorithm::HS384, &ValidationOptions::dangerous());
        assert!(maybe_res.is_ok());
    }