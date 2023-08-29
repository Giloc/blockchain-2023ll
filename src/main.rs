use std::io::{self, Write};
use std::fs::{File, OpenOptions};

mod modulos;

use modulos::merkle;
use modulos::block;

struct User {
    name: String,
    coins: u64
}

fn main() {
    let mut transactions: Vec<String> = Vec::new();
    let mut merkle_tree;
    let mut blocks: Vec<block::Block> = Vec::new();
    let mut block;
    let default_prev_hash = merkle::encrypt::encrypt(&String::from("0"));
    let default_nonce:i64 = 5;
    loop {
        println!("Do you want to finish the program? (y/n): ");
        let mut program = String::new();

        io::stdin().read_line(&mut program).expect("Failed to read line");

        program = program.trim().to_string();
        if program == "y"{
            break;
        }

        println!("Input sender user's name:");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let mut user1 = User{
            name: user_input,
            coins: 50
        };

        println!("Input receiver user's name:");

        let mut user_input2 = String::new();
        io::stdin().read_line(&mut user_input2).expect("Failed to read line");

        let mut user2 = User{
            name: user_input2,
            coins: 50
        };

        println!("Input amount to send:");
        let mut amount_input = String::new();
        io::stdin().read_line(&mut amount_input).expect("Failed to read line");

        let amount_number: u64 = amount_input.trim().parse().expect("invalid number");

        let result: String = trade(&mut user1, &mut user2, amount_number);

        transactions.push(result);

        if transactions.len() == 4 {
            merkle_tree = merkle::make_merkle_tree(&transactions);
            if blocks.len() == 0{
                block = block::create_block(&default_prev_hash, &merkle_tree.root.hash, default_nonce);
                blocks.push(block);
            }
            else{
                let prev_hash = block::get_block_hash(&blocks[blocks.len() - 1]);
                block = block::create_block(&prev_hash, &merkle_tree.root.hash, default_nonce);
                blocks.push(block);
            }

            let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("transactions.txt")
            .expect("file could not be open");

            let hash = block::get_block_hash(&blocks[blocks.len() - 1]);
            if let Err(e) = writeln!(file, "{}", hash){
                eprintln!("Error in file writing");
            }

            transactions.clear();
        }

        println!("coins user 1: {}", user1.coins);
        println!("coins user 2: {}", user2.coins);
    }
    
}

fn trade(requester: &mut User, receiver: &mut User, amount: u64) -> String {
    if requester.coins < amount{
        return String::new();
    }

    requester.coins = requester.coins - amount;
    receiver.coins = receiver.coins + amount;

    let tran = format!("${} De: {} -> {}", amount, requester.name.trim(), receiver.name.trim());

    return merkle::encrypt::encrypt(&tran);
}
