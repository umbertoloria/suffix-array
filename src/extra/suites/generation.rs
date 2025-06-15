use crate::files::fasta::save_fasta_with_content;
use rand::prelude::*;

const GENETIC_ALPHABET: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn main_generation() {
    let mut num_length = 700_000;

    let max_chars_per_iteration = 100_000;
    let num_lines_max = num_length / max_chars_per_iteration;

    let mut output = String::new();

    println!("Generating file with {} chars", num_length);
    while num_length > 0 {
        let num_chars_this_iteration = i32::min(max_chars_per_iteration, num_length);
        let mut i = 0;
        while i < num_chars_this_iteration {
            let rand_index = rand::random_range(0..GENETIC_ALPHABET.len());
            let rand_char = GENETIC_ALPHABET[rand_index];
            output.push(rand_char);

            i += 1;
            num_length -= 1;
        }
        let num_lines_left = num_length / max_chars_per_iteration;
        println!(
            " -> {}/{} lines: {} chars done, left {}",
            num_lines_left, num_lines_max, num_chars_this_iteration, num_length
        );
    }
    println!("Generated, now it's time to save...");

    let filepath = "generated/002_700.fasta";
    save_fasta_with_content(filepath.into(), output);

    println!("OK!");
}
