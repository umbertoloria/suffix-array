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
    let chunk_size_interval = (1, 50);
    // let chunk_size_interval = (5, 5);

    // Logging?
    let le = true;
    let lts = false;
    // let lts = true;

    // suite_complete_on_fasta_file("000", chunk_size_interval, 25, le, lts);
    // suite_complete_on_fasta_file("001", chunk_size_interval, 25, le, lts);
    // suite_complete_on_fasta_file("002_mini", chunk_size_interval, 30, le, lts);
    suite_complete_on_fasta_file("002_70", chunk_size_interval, 70_000, le, lts);
    // suite_complete_on_fasta_file("002_700", chunk_size_interval, 2_100_000, le, lts);
    // suite_complete_on_fasta_file("002_7000", chunk_size_interval, le, lts);
    // suite_complete_on_fasta_file("002_70000", chunk_size_interval, le, lts);
}
