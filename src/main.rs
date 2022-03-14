use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};
//use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename which will be the second element of the args
    let filename = args().nth(1).ok_or("Expected filename")?;
    // Open the file
    let file = File::open(filename.clone())?;
    // Fail early if the file is empty
    if file.metadata()?.len() == 0 {
        return Err("File is empty".into());
    }
    // Wrap the file in a BufReader
    let file = BufReader::new(file);
    let sum: i64 = file
        .lines() // Get the lines of the file
        .map(|l| {
            // Parse each line as an i64
            l.and_then(|v| {
                v.parse::<i64>()
                    // Map the error to std::io::Error to make `and_then` work
                    .map_err(|e| IoError::new(ErrorKind::InvalidData, e))
            })
        })
        .collect::<Result<Vec<i64>, IoError>>() // Collect the iterator as a Result containing the Vec<i64>
        .map_err(|e| {
            // Turn the error into something nicer to output
            format!(
                "There was a problem parsing the numbers in the file `{}`: {}",
                filename,
                e.to_string()
            )
        })?
        .iter() // Convert to iterator and sum
        .sum();
    println!("The sum is {:?}", sum);
    Ok(())
}
