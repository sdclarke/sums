use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind};

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
                    // I must admit I don't totally understand why this is necessary, I think I
                    // gathered it was requrired by the definition of `and_then` that the Result
                    // returned has the same kind of Error as the original?
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

    /*
     * Here is what I did originally, which to me seems to me to be more readable and
     * understandable due to the weird stuff I had to do with the map method above. If the lines
     * came out of the `.lines()` method of the BufReader just as plain Strings rather than Results
     * I think the map method would've been a lot cleaner and more understandable
     *
     * To be clear, these lines would replace the entire `let sum ...` statement
    let mut sum: i64 = 0;
    for line in file.lines() {
        let line = line?;
        let number: i64 = line.parse()?;
        sum += number;
    }
     */
}
