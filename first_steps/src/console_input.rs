// Inputs and Outputs console basic stuff
use std::io::{self, Write};

pub fn run(){
    // Reading line of input
    print!("Enter your name: ");
    io::stdout().flush().unwrap(); // flush because print! doesn't auto-flush

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim(); // Remove the newline character
    println!("Hello, {name}!");

    // Parsing input
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    match input.trim().parse::<i32>(){
        Ok(number) => println!("You entered: {number}"),
        Err(_) => println!("That's not a valid number. "),
    }
}