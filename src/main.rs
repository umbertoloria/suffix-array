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

    // Perform Logging?
    let pl = true;
    // let pl = false;

    // suite_complete_on_fasta_file("000", chunk_size_interval, 25, pl);
    // suite_complete_on_fasta_file("001", chunk_size_interval, 25, pl);
    // suite_complete_on_fasta_file("002_mini", chunk_size_interval, 30, pl);
    suite_complete_on_fasta_file("002_70", chunk_size_interval, 70_000, pl);
    // suite_complete_on_fasta_file("002_700", chunk_size_interval, 2_100_000, pl);
    // suite_complete_on_fasta_file("002_7000", chunk_size_interval, pl);
    // suite_complete_on_fasta_file("002_70000", chunk_size_interval, pl);
}
