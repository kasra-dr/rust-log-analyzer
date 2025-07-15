use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Collects command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Check whether the argument was given to the program or not
    if args.len() < 2 {
        // If no arguments were given, print a help message
        println!("Usage: log_analyzer <path_to_logfile>");
        println!("Analyzes a log file and provides a summary.");
    } else {
        // If there is an argument, treat it as a filename    
        let log_file_path = &args[1];
        println!("Analyzing file: {}", log_file_path);
        println!("---------------------------------");

        // Call new function to read file
        if let Err(e) = read_lines(log_file_path) { 
            eprintln!("Error reading file: {}", e);
        }
    }
}

// 'read_line' function takes a file path and reads it line by line
fn read_lines<P>(filename: P) -> io::Result<()>
where P: AsRef<Path>, {
    // Open the file
    let file = File::open(filename)?;

    // Create a buffer for optimal reading
    let reader = io::BufReader::new(file);

    // Iterate over each line of the file and print it
    for line in reader.lines() {
        // Line by line Checking file
        if let Ok(content) = line {
            println!("{}", content);
        }
    }
    Ok(())
}
