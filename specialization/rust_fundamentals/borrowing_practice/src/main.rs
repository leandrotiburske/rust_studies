// this script is used to practice the concept of borrowing in Rust

// define `own_vec` function, which takes a integer vector as input 
fn own_vec(vector: &mut Vec<i32>) {
    // append 10 to the vector and print it
    vector.push(10);
    println!("{:?}", vector);
}


fn own_integer(x: i32) {
    x + 1;
}

fn own_string(s: String) {
    println!("{}", s);
}

fn borrow_string(s: &mut String) {
    s.push_str(" Bye, world!");
    println!("{s}");
}

// borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// when you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
fn main() {
    let mut my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let my_int = 10;
    let mut my_string = String::from("Hello, world!");

    // this compiles no problem!
    own_integer(my_int);
    println!("{}", my_int);

    // check string before borrowing
    println!("{my_string}");
    borrow_string(&mut my_string);

    // check variable value after borrowing
    println!("{my_string}");
    own_string(my_string); // take ownership of my_string
    // this is using my_string which has also moved and is invalid
    //println!("{:?}", my_string); // this will not compile!

    own_vec(&mut my_vec);
    println!("{:?}", my_vec); 
}

// borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// by lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.