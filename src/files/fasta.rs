use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_fasta_content(filepath: &str) -> String {
    let file =
        File::open(filepath).expect(format!("Unable to read {} FASTA file", filepath).as_str());
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

pub fn save_fasta_with_content(filepath: String, whole_line: String) {
    let n = 70;
    // TODO: Impossible to save huge amounts of data with this approach...
    let mut content = whole_line
        .chars()
        .enumerate()
        .fold(String::new(), |acc, (i, c)| {
            if i != 0 && i % n == 0 {
                format!("{}\n{}", acc, c)
            } else {
                format!("{}{}", acc, c)
            }
        });
    content = format!(">GENERATED\n{}\n", content);
    fs::write(filepath, content).expect("Unable to write FASTA file");
}
