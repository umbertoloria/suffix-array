use crate::suites::similarity::main_similarity;
use suites::factorization::main_factorization;

mod jaccard;
mod ksliding;
mod suites;
mod factorization;
mod files;

fn main() {

    // FACTORIZATIONS
    main_factorization();

    // SIMILARITY
    main_similarity();
}
