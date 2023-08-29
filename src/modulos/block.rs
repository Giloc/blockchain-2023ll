use sha256::{digest};

#[derive(Debug)]
pub struct Block{
    prev_hash: String,
    root_hash: String,
    nonce: i64
}

pub fn create_block(previous_hash: &String, root_h: &String, non: i64) -> Block{
    return Block{
        prev_hash: previous_hash.clone(),
        root_hash: root_h.clone(),
        nonce: non
    };
}

pub fn get_block_hash(block: &Block) -> String{
    let block_string = format!("{:?}", *block);
    println!("block : {}", block_string);
    let hashed = digest(block_string);
    return hashed;
}