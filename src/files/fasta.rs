use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn get_fasta_content(filepath: String) -> String {
    let file = File::open(filepath.as_str())
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

pub fn save_fasta_with_content(filepath: String, whole_line: String) {
    let max_chars_in_line = 70;

    let string_length = whole_line.len();

    let mut f = File::create(filepath).expect("Unable to create file");
    f.write(format!(">GENERATED, with {} chars\n", string_length).as_bytes())
        .expect("Unable to write first line");

    let mut chars = whole_line.chars();

    let mut i = 0;
    while i < string_length {
        // Write one line at a time
        let mut curr_line = String::new();
        while let Some(curr_char) = chars.next() {
            curr_line.push(curr_char);
            if curr_line.len() < max_chars_in_line {
                // Ok.
            } else {
                break;
            }
        }
        i += curr_line.len();
        curr_line.push('\n');
        f.write(curr_line.as_bytes()).expect("Unable to write line");
        if curr_line.len() < max_chars_in_line {
            // No more chars.
            break;
        } else {
            // There are many chars to come.
        }
        println!(" > Written chars {}/{}", i, string_length);
    }
}
