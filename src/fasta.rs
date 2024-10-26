use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_fasta_content(filepath: &str) -> String {
    let file = File::open(filepath)
        .expect(format!("Unable to read {} FASTA file", filepath).as_str());
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    lines.next(); // Skip first line
    lines
        .map(|l| l.expect("Problem with a FASTA line"))
        .collect::<Vec<String>>()
        .join("")
}
