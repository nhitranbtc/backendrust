extern crate frank_jwt;
#[macro_use]
extern crate serde_json;

use std::env;
use std::path::{PathBuf};
use frank_jwt::{Algorithm, ValidationOptions, encode, decode, validate_signature};

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

    #[test]
    fn test_encoded_validate_signature_jwt_rs256() {
        let p1 = json!({
            "key1" : "val1",
            "key2" : "val2",
            "key3" : "val3"
        });

        let  header = json!({});
        let mut path = env::current_dir().unwrap();
        path.push("test");
        path.push("my_rsa_2048_key.pem");
        path.to_str().unwrap().to_string();

        let jwt1 = encode(header, &get_rsa_256_private_key_full_path(), &p1, Algorithm::RS256).unwrap();
        let maybe_res = validate_signature(&jwt1, &get_rsa_256_public_key_full_path(), Algorithm::RS256);
        assert!(maybe_res.unwrap());
    }

    #[test]
    fn test_decode_valid_jwt_rs256() {
        let p1 = json!({
            "key1" : "val1",
            "key2" : "val2"
        });

        let  header = json!({});
        let jwt1 = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJrZXkxIjoidmFsMSIsImtleTIiOiJ2YWwyIn0.DFusERCFWCL3CkKBaoVKsi1Z3QO2NTTRDTGHPqm7ctzypKHxLslJXfS1p_8_aRX30V2osMAEfGzXO9U0S9J1Z7looIFNf5rWSEcqA3ah7b7YQ2iTn9LOiDWwzVG8rm_HQXkWq-TXqayA-IXeiX9pVPB9bnguKXy3YrLWhP9pxnhl2WmaE9ryn8WTleMiElwDq4xw5JDeopA-qFS-AyEwlc-CE7S_afBd5OQBRbvgtfv1a9soNW3KP_mBg0ucz5eUYg_ON17BG6bwpAwyFuPdDAXphG4hCsa7GlXea0f7DnYD5e5-CA6O7BPW_EvjaGhL_D9LNWHJuDiSDBwZ4-IEIg".to_string();
        let (h1, p1) = decode(&jwt1, &get_rsa_256_public_key_full_path(), Algorithm::RS256, &ValidationOptions::dangerous()).unwrap();
        println!("\n{}",h1);
        println!("{}",p1);
        let jwt2 = encode(header, &get_rsa_256_private_key_full_path(), &p1, Algorithm::RS256).unwrap();
        let (h2, p2) = decode(&jwt2, &get_rsa_256_public_key_full_path(), Algorithm::RS256, &ValidationOptions::dangerous()).unwrap();
        println!("{}",h2);
        println!("{}",p2);
        assert_eq!(jwt1, jwt2);
    }

    fn get_rsa_256_private_key_full_path() -> PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("test");
        path.push("my_rsa_2048_key.pem");
        path.to_path_buf()
    }

    fn get_rsa_256_public_key_full_path() -> PathBuf {
        let mut path = env::current_dir().unwrap();
        path.push("test");
        path.push("my_rsa_public_2048_key.pem");
        path.to_path_buf()
    }