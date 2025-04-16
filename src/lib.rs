// src/lib.rs
use std::fs;
use std::path::Path;
use std::io::{BufWriter, Write};
use deunicode::deunicode;
use unicode_normalization::UnicodeNormalization;
use csv::ReaderBuilder;

pub fn clean_unicode(input: &str) -> String {
    input
        .nfkd()
        .collect::<String>()
        .chars()
        .filter(|c| c.is_ascii() || c.is_whitespace())
        .collect::<String>()
        .split('\n')
        .map(|line| deunicode(line).to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn clean_and_parse_csv<P: AsRef<Path>>(input_path: P, output_path: P) -> csv::Result<()> {
    let raw_csv = fs::read_to_string(&input_path).expect("Failed to read input file");

    let cleaned_csv = clean_unicode(&raw_csv);

    let mut writer = BufWriter::new(fs::File::create(&output_path).expect("Failed to create output file"));
    writer.write_all(cleaned_csv.as_bytes()).expect("Failed to write cleaned CSV");

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(output_path)?;

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
