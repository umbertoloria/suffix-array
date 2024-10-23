use crate::jaccard::jaccard_similarity;
use crate::ksliding::get_kmers;

mod lyndon;
mod jaccard;
mod ksliding;
mod debug;

fn main() {
    /*
    // LYNDON FACTORIZATION TEST
    let src = "umberto";
    let factors = lyndon::cfl_duval(src);
    for factor_byte in factors {
        let factor_str = String::from_utf8(factor_byte.to_vec()).unwrap();
        println!("{}", factor_str);
    }
    */

    // JACCARD ON K-MERS
    let mut bands = Vec::new();
    bands.push("radiohead");
    bands.push("slipknot");
    bands.push("avengedsevenfold");
    bands.push("systemofadown");
    bands.push("threedaysgrace");
    bands.push("breakingbenjamin");
    bands.push("mrbungle");
    bands.push("dreamtheater");
    bands.push("porcupinetree");
    bands.push("redhotchilipeppers");
    bands.push("sum41");
    bands.push("therecoveries");
    let k = 1; // Considering single letters for now.

    for i in 0..bands.len() {
        for j in i..bands.len() {
            let src1 = &bands[i];
            let src2 = &bands[j];

            let slides1 = get_kmers(src1, k);
            let slides2 = get_kmers(src2, k);
            // print_vec(&slides1);
            // print_vec(&slides2);

            // It is better to remove duplicates in-place?
            let unique_slides1 = vec_skip_duplicates(slides1);
            let unique_slides2 = vec_skip_duplicates(slides2);
            // print_vec(&unique_slides1);
            // print_vec(&unique_slides2);

            let calculated_jaccard_similarity = jaccard_similarity(&unique_slides1, &unique_slides2);
            println!("Similarity between \"{}\" and \"{}\" is: {}", src1, src2, calculated_jaccard_similarity);
        }
    }
}

fn vec_skip_duplicates(vec: Vec<&str>) -> Vec<&str> {
    let mut result = Vec::new();
    let mut first_position;
    for i in 0..vec.len() {
        first_position = true;
        for j in 0..i {
            if vec[i] == vec[j] {
                first_position = false;
                break;
            }
        }
        if first_position {
            result.push(vec[i]);
        }
    }
    result
}
