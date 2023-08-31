use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use sha256::{digest};

mod modulos;

use modulos::block;

struct User {
    name: String,
    coins: u64
}

fn main() {
    let mut block_list: Vec<block::Block> = Vec::new();
    let mut block = block::create_block(&digest(String::from("0")));
    let mut succ = true;
    
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

        let result = trade(&mut user1, &mut user2, amount_number, &mut block, &mut succ);

        if !succ {
            succ = true;
            continue;
        }

        if result {
            block_list.push(block);
            block = block::create_block(&block::get_block_hash(&block_list[block_list.len() - 1]));
        }

        println!("coins user 1: {}", user1.coins);
        println!("coins user 2: {}", user2.coins);
    }
    
}

fn trade(requester: &mut User, receiver: &mut User, amount: u64, block: &mut block::Block, success: &mut bool) -> bool {
    if requester.coins < amount{
        *success = false;
    }

    requester.coins = requester.coins - amount;
    receiver.coins = receiver.coins + amount;

    let block_ready = block::make_transaction(&requester.name, &receiver.name, 
        amount, block);

    return block_ready;
}
