use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// A struct to hold our analysis results
struct LogAnalysis {
    line_count: u32,
    status_codes: HashMap<u16, u32>,
    top_ips: HashMap<String, u32>,
}

impl LogAnalysis {
    // Creates a new and empty analysis instance
    fn new() -> Self {
        LogAnalysis {
            line_count: 0,
            status_codes: HashMap::new(),
            top_ips: HashMap::new(),
        }
    }

    // Prints the final summary
    fn print_summary(&self) {
        println!("\n--- Analysis Summary ---");
        println!("Total lines processed: {}", self.line_count);

        println!("\nHTTP Status Code Counts:");
        for (code, count) in &self.status_codes {
            println!(" - {code}: {count}");
        }
       
        println!("\nTop 5 IP Addresses:");
        for (ip, count) in self.top_ips.iter().take(5) {
            println!(" - {ip}: {count} requests");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: log_analyzer <path_to_logfile>");
    } else {
        let log_file_path = &args[1];
        println!("Analyzing file: {log_file_path}");

        let mut analysis = LogAnalysis::new();

        // A regex pattern for nginx log (IP, Status code)
        let re = Regex::new(r#"(?P<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}) - - \[(?P<timestamp>[^\]]+)\] "(?P<method>\w+) (?P<url>[^\s]+) [^"]+" (?P<status>\d{3}) .*"#).unwrap();

        if let Err(e) = read_and_analyze_lines(log_file_path, &mut analysis, &re) {
            eprintln!("Error reading file: {e}");
        } else {
            analysis.print_summary();
        }
    }
}

// This function reads and analyzes the file line by line
// The return type is correctly defined as io::Result<()>
fn read_and_analyze_lines<P>(filename: P, analysis: &mut LogAnalysis, re: &Regex) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for content in reader.lines().flatten() {
        if let Ok(content) = line {
            analysis.line_count += 1;

            // If the line matches our Regex pattern
            if let Some(captures) = re.captures(&content) {
                // Extract IP address
                if let Some(ip) = captures.name("ip") {
                    *analysis.top_ips.entry(ip.as_str().to_string()).or_insert(0) += 1;
                }

                // Extract Status Code
                if let Some(status_str) = captures.name("status") {
                    if let Ok(status_code) = status_str.as_str().parse::<u16>() {
                        *analysis.status_codes.entry(status_code).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    Ok(())
}
