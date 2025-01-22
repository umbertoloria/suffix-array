#![allow(warnings)]

use suites::factorization::main_factorization;
use suites::generation::main_generation;
use suites::similarity::main_similarity;
use suites::suffix::main_suffix;
use suites::suffix_array::main_suffix_array;

mod comparing;
mod factorization;
mod files;
mod suffix_array;
mod suites;

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
