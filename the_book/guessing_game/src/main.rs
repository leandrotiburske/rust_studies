// use standard input/output library and the Ordering type
use std::{cmp::Ordering, io};

// use the rand library to get random numbers
use::rand::Rng;

// define main function
fn main() {
    println!("Guess the number!");

    // get a random number between 1 and 100 from the user
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // ask the user for a number until it matches the secret number
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // read input into the 'guess' variable and print error if
        // not in the expected format
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {guess}");

        // remove whitespaces from 'guess' and make it a number
        // if not a number, skip comparing to secret number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // compare to secret number and break if the user win
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}