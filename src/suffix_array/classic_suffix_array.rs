use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::time::{Duration, Instant};

pub struct ClassicSuffixArrayComputationResults<'a> {
    pub suffix_array: Vec<usize>,
    pub suffix_array_pairs: Vec<(usize, &'a str)>,
    pub duration: Duration,
}
pub fn compute_classic_suffix_array(
    src: &str,
    debug_verbose: bool,
) -> ClassicSuffixArrayComputationResults {
    let before = Instant::now();

    let mut suffix_array_pairs = Vec::new();
    // Create array of global suffixes
    for i in 0..src.len() {
        suffix_array_pairs.push((i, &src[i..]));
    }
    // Create sort by comparing global suffixes
    sort_pair_vector_of_indexed_strings(&mut suffix_array_pairs);
    // Extracting only indexes of previous array of pairs
    let mut suffix_array_indexes = Vec::new();
    for &(index, _) in &suffix_array_pairs {
        suffix_array_indexes.push(index);
    }
    let after = Instant::now();
    let duration = after - before;

    if debug_verbose {
        for &(index, suffix) in &suffix_array_pairs {
            println!(" > SUFFIX_ARRAY [{:3}] = {}", index, suffix);
        }
    }

    // println!("Total time: {}", duration.as_secs_f32());

    ClassicSuffixArrayComputationResults {
        suffix_array: suffix_array_indexes,
        suffix_array_pairs,
        duration,
    }
}
