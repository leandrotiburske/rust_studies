// use input/output and vector standard libraries 
use std::{io, vec::Vec};


// define function that sums up a vector of integers
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
   for number in numbers {
        result += number;
    }
    result
}

// receives a list of integers as input and returns a float as result
fn average(numbers: &[i32]) -> f32 {
    let mut result: f32 = 0.0;

    // calls `sum` function to get sum of integers
    let sum = sum(&numbers) as f32;

    // if the list is not empty,
    if numbers.len() != 0 {

        // calculate the average by dividing sum by the quantity of numbers
        result = sum / (numbers.len() as f32);
    }
    result
}

// define main function
fn main() {
    // variable that stores the user's numbers
    let mut vector = Vec::new();

    // asks the user for a number until they type 'done'
    loop {
        println!("Type a number: ");
        let mut number = String::new();
        
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read");
        
        if number.trim() == "done" {
            break;
        }

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        vector.push(number);
        println!("You typed: {}. When you're done, type 'done'.", number);
    }

    // calls 'sum' function with the user's number vector
    let result = sum(&vector);

    //prints the result
    println!("The sum is {}", result);

    let result = average(&vector);
    println!("The average is {}", result);
}