/* Extend this example */
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let data: Vec<i64> = (1..10000).collect();

    println!("Start parallelized sum...");
    let start = Instant::now();

    let parallel_sum: i64 = data
        .par_iter() // Specify the type
        .map(|x| x * x)
        .sum();

    println!("Total time: {:?}", start.elapsed());

    println!("Parallel sum: {}", parallel_sum);

    println!("Start sequential sum...");
    let start = Instant::now();

    let parallel_sum: i64 = data
        .iter() // Specify the type
        .map(|x| x * x)
        .sum();

    println!("Total time: {:?}", start.elapsed());

    println!("Parallel sum: {}", parallel_sum);
}
