use crate::suffix_array::monitor::ExecutionOutcome;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct ExecutionOutcomeFileFormat {
    compares_using_rules: usize,
    compares_using_strcmp: usize,
    compares_using_one_cf: usize,
    compares_using_two_cf: usize,
}

pub fn log_execution_outcome(execution_outcome: &ExecutionOutcome, filepath: String) {
    let file_format = ExecutionOutcomeFileFormat {
        compares_using_rules: execution_outcome.compares_using_rules,
        compares_using_strcmp: execution_outcome.compares_using_strcmp,
        compares_using_one_cf: execution_outcome.compares_with_one_cf,
        compares_using_two_cf: execution_outcome.compares_with_two_cfs,
    };
    let json = serde_json::to_string_pretty(&file_format).unwrap();
    let mut file = File::create(filepath).expect("Unable to create file");
    file.write(json.as_bytes())
        .expect("Unable to write JSON string");
    file.flush().expect("Unable to flush file");
}
