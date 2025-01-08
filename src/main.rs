use suites::suffix::main_suffix;
use suites::suffix_array::main_suffix_array;
use suites::factorization::main_factorization;
use suites::similarity::main_similarity;
use suites::generation::main_generation;

mod comparing;
mod factorization;
mod files;
mod suffix_array;
mod suites;

fn main() {
    // SUFFIX
    // main_suffix()

    // FILES GENERATION
    main_generation();

    // SUFFIX ARRAY
    // main_suffix_array();

    // FACTORIZATIONS
    // main_factorization();

    // SIMILARITY
    // main_similarity();
}
