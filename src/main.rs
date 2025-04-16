use std::env;
use std::path::Path;
use clean_and_parse_csv::clean_and_parse_csv;

fn main() {
    // Get input and output file paths from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_csv_path> <output_csv_path>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Call the clean_and_parse_csv function
    if let Err(e) = clean_and_parse_csv(Path::new(input_path), Path::new(output_path)) {
        eprintln!("Error processing CSV: {}", e);
        std::process::exit(1);
    }

    println!("CSV cleaned and parsed successfully!");
}