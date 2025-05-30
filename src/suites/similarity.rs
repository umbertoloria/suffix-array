use crate::comparing::jaccard::jaccard_similarity_via_kmers;
use crate::files::fasta::get_fasta_content;
use std::time::Instant;

pub fn main_similarity() {
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
            let src1 = bands[i];
            let src2 = bands[j];

            let calculated_jaccard_similarity = jaccard_similarity_via_kmers(src1, src2, k);
            println!(
                "Similarity between \"{}\" and \"{}\" is: {}",
                src1, src2, calculated_jaccard_similarity
            );
        }
    }

    // JACCARD ON K-MERS OF FILES
    let filepath1 = "in/1.fasta";
    let contents1_buf = get_fasta_content(filepath1.into());
    let contents1 = contents1_buf.as_str();

    let filepath2 = "in/2.fasta";
    let contents2_buf = get_fasta_content(filepath2.into());
    let contents2 = contents2_buf.as_str();

    let before = Instant::now();
    for k in 1..141 {
        let calculated_jaccard_similarity = jaccard_similarity_via_kmers(contents1, contents2, k);
        println!("Similarity k={} is: {}", k, calculated_jaccard_similarity);
    }
    let after = Instant::now();
    println!("Total time: {}", (after - before).as_secs_f32());
}
