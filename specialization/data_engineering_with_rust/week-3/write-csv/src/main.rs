use csv::{Reader, Writer};
use std::{error::Error, io, process};

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_reader(io::stdin()); // read CSV
    let mut wtr = Writer::from_path("discounted_fruits.csv")?; // CSV file to be written
    wtr.write_record(["Fruit", "Price_after_discount"])?; // Write header

    for result in rdr.records() {
        let record = result?; // Get row
        let record: Vec<&str> = record.into_iter().collect(); // Create &str vector from row
        let discount: f32 = record[1].trim().parse::<f32>().unwrap() * 0.9; // Apply 10% discount
        wtr.write_record(&[record[0].to_string(), discount.to_string()])?; // Write final row
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
