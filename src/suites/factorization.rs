use crate::factorization::cfl::cfl;
use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use std::fs;
use std::process::exit;

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

    // ..WITH FILES
    for index in 1..1553 {
        println!("Factorizing FASTA file: {}", index);
        print_kmers_from_fasta_to_file_number(index, "cfl");
        print_kmers_from_fasta_to_file_number(index, "icfl");
    }
}

fn print_kmers_from_fasta_to_file_number(index: usize, fact_alg_of_choice: &str) {
    // Read FASTA file
    let file_path_read = format!("in/{}.fasta", index);
    let contents_buf = get_fasta_content(file_path_read);
    let contents = contents_buf.as_str();

    // Factorization
    let mut kmers = Vec::new();
    match fact_alg_of_choice {
        "cfl" => {
            kmers = cfl(contents);
        }
        "icfl" => {
            kmers = icfl(contents);
        }
        _ => {
            exit(0x0100);
        }
    }

    // Write file
    let mut write_text = String::new();
    for kmer in &kmers {
        write_text.push_str(format!("{}\n", kmer).as_str());
    }
    let file_path_write = format!("out/{}-{}.txt", index, fact_alg_of_choice);
    fs::write(file_path_write.as_str(), write_text.as_str()).unwrap();
}
