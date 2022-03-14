use std::env::args;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the filename which will be the second element of the args
    let filename = args().nth(1).ok_or("Expected filename")?;
    // Read the contents of the file to a string
    // Give an nice error message if it fails
    let mut contents = fs::read_to_string(filename.clone()).or(Err(format!(
        "Could not read from the file `{}`, does it exist with the correct permissions and contain valid UTF-8?",
        filename
    )))?;
    // Remove any trailing whitespace
    contents = contents.trim_end().to_string();
    // Check for an empty file
    if contents.is_empty() {
        return Err("The file was empty or contained only whitespace".into());
    }
    let sum: i64 = contents
        .split('\n') // Split into lines
        .map(|e| e.parse::<i64>()) // Parse each line as i64
        .collect::<Result<Vec<i64>, ParseIntError>>() // Collect iterator as a `Result` containing a Vec<i64> or and Error
        .or(Err("Encountered something which was not an integer"))? // Check the error from parsing
        .iter() // Turn Vec to Iterator
        .sum(); // Sum up elements of the iterator
    println!("The sum is {:?}", sum);
    Ok(())
}
