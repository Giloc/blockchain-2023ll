use sha256::{digest};
use std::io::{Write};
use std::fs::{OpenOptions};

mod merkle;

pub struct Block{
    header: Header,
    body: Body
}

#[derive(Debug)]
struct Header{
    prev_hash: String,
    root_hash: String,
    nonce: i64
}

#[derive(Debug)]
struct Body{
    pub transactions: Vec<Transaction>
}

#[derive(Debug)]
struct Transaction{
    sender: String,
    receiver: String,
    amount: u64
}

pub fn create_block(prev: &String) -> Block{

    let head = Header{
        prev_hash: prev.to_string(),
        root_hash: String::new(),
        nonce: 0
    };

    let bod = Body{
        transactions: Vec::new()
    };

    return Block { header: head, body: bod }
}

pub fn get_block_hash(block: &Block) -> String{
    let header = &block.header;
    let block_string = format!("{:?}", *header);
    let hashed = digest(block_string);
    return hashed;
}

pub fn make_transaction(sen: &String, rec: &String, amo: u64, block: &mut Block) -> bool {
    let tran = Transaction{
        sender: sen.clone(),
        receiver: rec.clone(),
        amount: amo
    };

    block.body.transactions.push(tran);
    if block.body.transactions.len() == 4{
        finish_block(block);
        print_block(block);
        return true;
    }
    return false;
}

fn finish_block(block: &mut Block){
    let root_hash = get_root_hash(&block.body.transactions);

    block.header.root_hash = root_hash;
    mine_block(block);
}

fn print_block(block: &Block){  
    let header = &block.header;
    let block_string = format!("{:?}", *header);

    println!("block header: {}", block_string);

    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("transactions.txt")
            .expect("file could not be open");

    let hash = get_block_hash(block);

    if let Err(e) = writeln!(file, "{}", hash){
        eprintln!("Error in file writing");
    }

}

fn get_root_hash(transactions: &Vec<Transaction>) -> String{
    let mut hashed_transactions = Vec::new();
    for transaction in transactions.iter() {
        let tran_string = format!("{:?}", *transaction);
        hashed_transactions.push(digest(tran_string));
    }

    return merkle::make_merkle_tree(&hashed_transactions).root.hash;
}

fn mine_block(block: &mut Block) {
    let proof = String::from("000");
    let mut current_hash = get_block_hash(block);

    while !current_hash.chars().take(3).collect::<String>().starts_with(&proof) {
        block.header.nonce += 1;
        current_hash = get_block_hash(block);
    }
}