#![allow(warnings)]

use suites::suffix_array::main_suffix_array;

mod comparing;
mod factorization;
mod files;
mod plot;
mod suffix_array;
mod suites;
mod utils;

fn main() {
    // TODO: Control this main with CLI Interface with Arguments

    // SUFFIX
    // main_suffix();

    // FILES GENERATION
    // main_generation();

    // SUFFIX ARRAY
    main_suffix_array();

    // FACTORIZATIONS
    // main_factorization();

    // SIMILARITY
    // main_similarity();
}
