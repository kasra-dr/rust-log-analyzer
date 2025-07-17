use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// A struct to hold our analysis results
struct LogAnalysis {
    line_count: u32,
    log_levels: HashMap<String, u32>,
}

impl LogAnalysis {
    // Creates a new, empty analysis instance
    fn new() -> Self {
        LogAnalysis {
            line_count: 0,
            log_levels: HashMap::new(),
        }
    }

    // Prints the final summary
    fn print_summary(&self) {
        println!("\n--- Analysis Summary ---");
        println!("Total lines processed: {}", self.line_count);
        println!("\nLog level counts:");
        for (level, count) in &self.log_levels {
            println!("  - {}: {}", level, count);
        }
        println!("----------------------");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: log_analyzer <path_to_logfile>");
    } else {
        let log_file_path = &args[1];
        println!("Analyzing file: {}", log_file_path);

        let mut analysis = LogAnalysis::new();

        if let Err(e) = read_and_analyze_lines(log_file_path, &mut analysis) {
            eprintln!("Error reading file: {}", e);
        } else {
            analysis.print_summary();
        }
    }
}

// This function reads and analyzes the file line by line
// The return type is correctly defined as io::Result<()>
fn read_and_analyze_lines<P>(filename: P, analysis: &mut LogAnalysis) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(content) = line {
            analysis.line_count += 1;

            if content.contains("INFO") {
                *analysis.log_levels.entry("INFO".to_string()).or_insert(0) += 1;
            } else if content.contains("WARNING") {
                *analysis.log_levels.entry("WARNING".to_string()).or_insert(0) += 1;
            } else if content.contains("ERROR") {
                *analysis.log_levels.entry("ERROR".to_string()).or_insert(0) += 1;
            }
        }
    }
    Ok(())
}
