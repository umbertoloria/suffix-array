use crate::lyndon::{cfl, icfl};

pub fn main_factorization() {

    // LYNDON FACTORIZATION
    let src = "umberto";
    println!("Source (CFL): {}", src);
    let factors = cfl(src);
    for factor in factors {
        println!("{}", factor);
    }
    println!();

    // INVERSE LYNDON FACTORIZATION
    let src = "aaaba";
    println!("Source (ICFL): {}", src);
    let factors = icfl(src);
    for factor in factors {
        println!("{}", factor);
    }
    println!();
}