use indexmap::IndexSet;
use clap::Parser;
use salad_cli::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Leandro Tiburske <tiburskeleandro@gmail.com>",
    about = "Fruit salad generator"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,

    #[clap(short, long)]
    fruits: Option<Vec<String>>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let fruits: Vec<String>;

    if opts.fruits.is_some() {
        fruits = opts.fruits.unwrap();
        println!("{:?}", fruits);
    } else {
        fruits = [].to_vec();
    }

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    let result = create_fruit_salad(num_fruits);
    let mut result = result.iter().map(|s| &s[..]).collect::<Vec<&str>>();
    if fruits.len() > 0 {
        result.append(&mut fruits.iter().map(|s| &s[..]).collect::<Vec<&str>>());
    }

    // Sort the fruit salad alphabetically
    result.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    // Remove duplicated fruits
    let result:IndexSet<&str> = result.iter().cloned().collect();

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits, result
    );
}
