use rand::Rng;
use std::io;

fn receive_bitcoin() {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let number: u32 = rng.gen_range(0..10);
    println!("You have received {} bitcoins", number);
}

fn send_bitcoin() {
    let receivers = vec!["Alice", "Bob", "Charlie", "Dave"];
    println!("Please select a receiver:");
    let mut receiver = String::new();
    io::stdin()
        .read_line(&mut receiver)
        .expect("Failed to read line");

    if receivers.contains(&receiver.trim()) {
        println!("Please enter the amount of bitcoins to send:");
        let mut amount: String = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");
        let amount: u32 = amount.trim().parse().expect("Please enter a number");

        println!("Sending {} bitcoins to {}", amount, receiver);
    } else {
        println!("Receiver not found");
    }
}

fn console() {
    println!("Welcome to the bitcoin console!");

    loop {
        println!(
            "Please enter your command (s) for send or (r) for receive bitcoin or (q) to quit >"
        );
        let mut command: String = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command: &str = command.trim();

        if command == "s" {
            send_bitcoin();
        } else if command == "r" {
            receive_bitcoin();
        } else if command == "q" {
            break;
        } else {
            println!("Invalid command");
        }
    }
}

fn main() {
    console();
}
