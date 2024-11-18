use crate::suites::suffix::main_suffix;
use suites::factorization::main_factorization;
use crate::suites::similarity::main_similarity;

mod suites;
mod factorization;
mod files;
mod comparing;

fn main() {
    // SUFFIX
    main_suffix()

    // FACTORIZATIONS
    // main_factorization();

    // SIMILARITY
    // main_similarity();
}
