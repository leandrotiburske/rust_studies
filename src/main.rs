// use standard input/output library
use std::io;

// use the rand library to get random numbers
use::rand::Rng;

// define main function
fn main() {
    println!("Guess the number!");

    // get a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // print random number
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}