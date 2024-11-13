use crate::main_factorization::main_factorization;
use crate::suites::similarity::main_similarity;

mod lyndon;
mod jaccard;
mod ksliding;
mod debug;
mod vector;
mod fasta;
mod main_factorization;
mod suites;

fn main() {

    // FACTORIZATIONS
    main_factorization();

    // SIMILARITY
    // main_similarity();
}
