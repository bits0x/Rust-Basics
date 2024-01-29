use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing Game");
    let random = rand::thread_rng().gen_range(1, 101);

    println!("Please guess a number");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid integer in range 1 - 100".red());
                continue;
            }
        };

        match guess.cmp(&random) {
            Ordering::Greater => println!("{}", "Guessed no. is big, Try Again".red()),
            Ordering::Less => println!("{}", "Guessed no. is small, Try Again".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}
