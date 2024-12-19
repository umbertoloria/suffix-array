use crate::suites::suffix::main_suffix;
use crate::suites::suffix_array::main_suffix_array;
use suites::factorization::main_factorization;
use crate::suites::similarity::main_similarity;

mod comparing;
mod factorization;
mod files;
mod suffix_array;
mod suites;

fn main() {
    // SUFFIX
    // main_suffix()

    // SUFFIX ARRAY
    main_suffix_array();

    // FACTORIZATIONS
    // main_factorization();

    // SIMILARITY
    // main_similarity();
}
