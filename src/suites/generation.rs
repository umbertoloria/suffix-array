use crate::files::fasta::save_fasta_with_content;
use rand::prelude::*;

const GENETIC_ALPHABET: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn main_generation() {
    let num_length = 150;

    let mut output = String::new();
    for _ in 0..num_length {
        let rand_index = rand::random_range(0..GENETIC_ALPHABET.len());
        let rand_char = GENETIC_ALPHABET[rand_index];
        output.push(rand_char);
    }

    let filepath = "generated/000.fasta";
    save_fasta_with_content(filepath.into(), output);
}
