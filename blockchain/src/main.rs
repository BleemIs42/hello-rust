#[macro_use]
extern crate serde_derive;

use std::{
    io::{self, Write},
    process,
};

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("Need a integer");
    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);
    loop {
        println!("\nMenu");
        println!("  1) New transaction");
        println!("  2) Mine Block");
        println!("  3) Change Difficulty");
        println!("  4) Change Reward");
        println!("  0) Exit");
        print!("\nEnter you choice: ");

        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                print!("exiting");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter a sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Enter a receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let ret = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match ret {
                    true => println!("New transaction added"),
                    false => println!("New transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let ret = chain.generate_new_block();
                match ret {
                    true => println!("New block generated successfully"),
                    false => println!("New block generation failed"),
                }
            }
            3 => {
                let mut diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut diff);
                let ret = chain.update_difficulty(diff.trim().parse().unwrap());
                match ret {
                    true => println!("Update difficulty: {}", diff),
                    false => println!("Update difficulty failed"),
                }
            }
            4 => {
                let mut reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut reward);
                let ret = chain.update_reward(reward.trim().parse().unwrap());
                match ret {
                    true => println!("Update reward: {}", reward),
                    false => println!("Update reward failed"),
                }
            }
            _ => println!("Invalid option, please retry!"),
        };
    }
}
