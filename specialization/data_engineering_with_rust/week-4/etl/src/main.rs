// ETL Example

use std::error::Error;
use csv::Writer;

#[derive(Debug)]
struct RawData {
    id: u32,
    value: i32,
}

#[derive(Debug)]
struct CleanData {
    id: u32,
    value: i32,
}

fn main() {
    let raw = vec![
        RawData { id: 1, value: 10 },
        RawData { id: 2, value: 122 }
    ];

    let cleaned = match extract_transform_load(raw) {
        Err(error) => panic!("Error when transforming data: {:?}", error),
        Ok(vector) => vector,
    }; 

    println!("Clean Data: Id - {:?} Value - {:?}", cleaned[1].id, cleaned[1].value); // Accessing the fields

    let mut total: i32 = 0;

    for cleaned_data in &cleaned {
        total += cleaned_data.value as i32;
    }

    let average: i32 = total / (cleaned.len() as i32); 

    println!("Sum of values: {:?}", total);
    println!("Values average: {:?}", average);

    println!("Writing into CSV file...");
    match write_csv(cleaned) {
        Err(error) => panic!("Could not write CSV file: {:?}", error),
        Ok(file) => file, 
    };
}

// Perform ETL process 
fn extract_transform_load(raw: Vec<RawData>) -> Result<Vec<CleanData>, Box<dyn Error>> {
    raw.into_iter() 
    .map(|r| {
        Ok(CleanData {
            id: r.id,
            value: r.value.max(0).min(100),
        })
    })
        .collect()
    }

fn write_csv(cleaned: Vec<CleanData>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data.csv")?;
    wtr.write_record(&["id", "value"])?;
    for i in cleaned {
        wtr.write_record(&[i.id.to_string(), i.value.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}