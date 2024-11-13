use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_fasta_content(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect(format!("Unable to read {} FASTA file", filepath).as_str());
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    lines.next(); // Skip first line (because it's the "header" of the FASTA format file)

    let mut result = String::new();
    while let Some(line_result) = lines.next() {
        let line_string = line_result.unwrap();
        let line_str = line_string.as_str();
        result.push_str(line_str);
    }
    result
}
