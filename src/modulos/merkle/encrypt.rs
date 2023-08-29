use sha256::{digest, try_digest};
use std::path::Path;

pub fn encrypt(input: &String) -> String {
    
    // let value = digest(input);
    return digest(input);
    // let inpu = Path::new("/home/giloc/university/blockchain/Encrypt/encrypt/src/file.json");
    // let val = try_digest(inpu).unwrap();
}