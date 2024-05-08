use clap::Parser;
use salad_cli::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    create_fruit_salad(num_fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        create_fruit_salad(num_fruits).sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()))
    );
}
