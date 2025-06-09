use std::fs::File;
use std::io::Write;

pub fn log_factorization(
    factor_indexes: &Vec<usize>,
    icfl_indexes: &Vec<usize>,
    str: &str,
    filepath: String,
) {
    let mut file = File::create(filepath).expect("Unable to create file");
    let mut content = String::new();

    let str_length = str.len();
    for i_factor in 0..factor_indexes.len() - 1 {
        let curr_fact_index = factor_indexes[i_factor];
        let next_fact_index = factor_indexes[i_factor + 1];
        let curr_fact = &str[curr_fact_index..next_fact_index];
        if icfl_indexes.contains(&curr_fact_index) {
            content.push_str(&format!("icfl > {curr_fact}\n"));
        } else {
            content.push_str(&format!("  cf > {curr_fact}\n"));
        }
    }
    let last_fact_index = factor_indexes[factor_indexes.len() - 1];
    let curr_fact = &str[last_fact_index..str_length];
    if icfl_indexes.contains(&last_fact_index) {
        content.push_str(&format!("icfl > {curr_fact}\n"));
    } else {
        content.push_str(&format!("  cf > {curr_fact}\n"));
    }

    file.write(content.as_bytes())
        .expect("Unable to write content");
    file.flush().expect("Unable to flush file");
}
