use std::io::{self, Write};
use std::fs::File;

mod modulos;

use modulos::merkle;

struct User {
    name: String,
    coins: u64
}

fn main() {
    let mut transactions: Vec<String> = Vec::new();
    let mut merkle_tree; 
    loop {
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
            let mut file = File::create("transaction.txt").expect("Error creating file");

            file.write_all(merkle_tree.root.hash.as_bytes()).expect("write operation failed");
            break;
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
