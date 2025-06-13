#![allow(warnings)]

use crate::suffix_array::suite::suite_complete_on_fasta_file;

mod extra;
mod factorization;
mod files;
mod plot;
mod suffix_array;
mod suites;
mod utils;

fn main() {
    // TODO: Control this main with CLI Interface with Arguments
    // OLD SUITES
    // main_generation();
    // main_factorization();

    // Chunk Size Interval
    let chunk_size_vec_000 = create_chunk_size_interval_and_none(2, 7);
    let chunk_size_vec = create_chunk_size_interval(1, 50);
    // let chunk_size_vec = create_chunk_size_interval(5, 200);
    // let chunk_size_vec = create_chunk_size_interval(5, 30);
    // let chunk_size_vec = create_chunk_size_interval(5, 5);
    // let chunk_size_vec = create_chunk_size_of_thousands_with_steps(1, 70);
    // let chunk_size_none_list = vec![None];

    // Logging?
    let le = true;
    let lftsa = false;
    // let lftsa = true;

    suite_complete_on_fasta_file("000", &chunk_size_vec_000, 25, le, lftsa);
    // suite_complete_on_fasta_file("001", &chunk_size_vec, 25, le, lftsa);
    // suite_complete_on_fasta_file("002_mini", &chunk_size_vec, 30, le, lftsa);
    // suite_complete_on_fasta_file("002_70", &chunk_size_vec, 70_000, le, lftsa);
    // suite_complete_on_fasta_file("002_70", &chunk_size_vec, 200_000, le, lftsa);
    // suite_complete_on_fasta_file("002_70", &chunk_size_none_list, 200_000, le, lftsa);
    // suite_complete_on_fasta_file("002_700", &chunk_size_vec, 2_100_000, le, lftsa);
    // suite_complete_on_fasta_file("002_7000", &chunk_size_vec, 50_000_000, le, lftsa);
    // suite_complete_on_fasta_file("002_70000", &chunk_size_vec, le, lftsa);
}

fn create_chunk_size_interval(min: usize, max: usize) -> Vec<Option<usize>> {
    (min..=max).map(|x| Some(x)).collect()
}

fn create_chunk_size_interval_and_none(min: usize, max: usize) -> Vec<Option<usize>> {
    let mut result = create_chunk_size_interval(min, max);
    result.push(None);
    result
}

fn create_chunk_size_of_thousands_with_steps(min: usize, max: usize) -> Vec<Option<usize>> {
    (min..=max)
        .map(|x| (x * 1_000, x * 1_000 + 250, x * 1_000 + 500, x * 1_000 + 750))
        .flat_map(|a| vec![a.0, a.1, a.2, a.3])
        .map(|x| Some(x))
        .collect()
}
