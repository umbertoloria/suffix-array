use crate::suites::similarity::main_similarity;
use suites::factorization::main_factorization;

mod suites;
mod factorization;
mod files;
mod comparing;

fn main() {

    // FACTORIZATIONS
    main_factorization();

    // SIMILARITY
    main_similarity();
}
