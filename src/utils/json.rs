use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub fn dump_json_in_file<T: Serialize>(file_format: &T, filepath: String) {
    let json = serde_json::to_string_pretty(file_format).unwrap();
    let mut file = File::create(filepath).expect("Unable to create file");
    file.write(json.as_bytes())
        .expect("Unable to write JSON string");
    file.flush().expect("Unable to flush file");
}
