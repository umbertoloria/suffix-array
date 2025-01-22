use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use crate::suffix_array::chunking::{
    get_custom_factor_strings_from_custom_indexes, get_custom_factors, get_factor_list,
    get_indexes_from_factors, get_is_custom_vec,
};
use crate::suffix_array::prefix_trie::create_prefix_trie;
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::time::{Duration, Instant};

pub fn main_suffix_array() {
    let src = get_fasta_content("generated/001.fasta".into());
    let src_str = src.as_str();
    // println!("STRING={}", src_str);

    // INNOVATIVE SUFFIX ARRAY
    let innovative_suffix_array_computation = compute_innovative_suffix_array(src_str, false);
    let wbsa = innovative_suffix_array_computation.suffix_array;
    println!("INNOVATIVE SUFFIX ARRAY CALCULATION");
    println!(
        " > Duration: {} seconds",
        innovative_suffix_array_computation.duration.as_secs_f64()
    );
    // println!(" > Suffix Array: {:?}", wbsa);

    // CLASSIC SUFFIX ARRAY
    println!();
    let classic_suffix_array_computation = compute_classic_suffix_array(src_str, false);
    let classic_suffix_array = classic_suffix_array_computation.suffix_array;
    println!("CLASSIC SUFFIX ARRAY CALCULATION");
    println!(
        " > Duration: {} seconds",
        classic_suffix_array_computation.duration.as_secs_f64()
    );
    // println!(" > Suffix Array: {:?}", classic_suffix_array);

    // VERIFICATION
    println!();
    if wbsa.len() != classic_suffix_array.len() {
        println!("Wanna Be Suffix Array is insufficient in size");
    } else {
        let mut i = 0;
        while i < classic_suffix_array.len() {
            let sa_item = classic_suffix_array[i];
            let wbsa_item = wbsa[i];
            if wbsa_item != sa_item {
                println!("Wanna Be Suffix Array is insufficient: element [{}] should be \"{}\" but is \"{}\"", i, sa_item, wbsa_item);
                break;
            }
            i += 1;
        }
        if i == classic_suffix_array.len() {
            println!("Wanna Be Suffix Array is PERFECT :)");
        }
    }
}

// CLASSIC SUFFIX ARRAY
struct ClassicSuffixArrayComputationResults<'a> {
    suffix_array: Vec<usize>,
    suffix_array_pairs: Vec<(usize, &'a str)>,
    duration: Duration,
}
fn compute_classic_suffix_array(
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

    if debug_verbose {
        for (index, suffix) in &suffix_array_pairs {
            println!(" > SUFFIX_ARRAY [{:3}] = {}", index, suffix);
        }
    }

    // println!("Total time: {}", duration.as_secs_f32());

    ClassicSuffixArrayComputationResults {
        suffix_array: suffix_array_indexes,
        suffix_array_pairs,
        duration: after - before,
    }
}

// INNOVATIVE SUFFIX ARRAY
struct InnovativeSuffixArrayComputationResults {
    suffix_array: Vec<usize>,
    duration: Duration,
}
fn compute_innovative_suffix_array(
    str: &str,
    debug_verbose: bool,
) -> InnovativeSuffixArrayComputationResults {
    let before = Instant::now();

    let src_length = str.len();

    // Compute ICFL
    let factors = icfl(str);

    let chunk_size = 3;
    // TODO: Simplify algorithms by having string length as last item of these Factor Index vectors
    let icfl_indexes = get_indexes_from_factors(&factors);
    let custom_indexes = get_custom_factors(&icfl_indexes, chunk_size, src_length);
    let custom_factors = get_custom_factor_strings_from_custom_indexes(str, &custom_indexes);

    // TODO: Optimize both functions and rename them (and variables)
    // Factor List: [Source Char Index] => True if it's part of the last Custom Factor of an
    //                                     ICFL Factor, so it's a Local Suffix
    let is_custom_vec = get_is_custom_vec(&icfl_indexes, src_length, chunk_size);

    // Factor List: [Source Char Index] => ICFL Factor Index of that
    let factor_list = get_factor_list(&icfl_indexes, src_length);

    if debug_verbose {
        println!("chunk_size={}", chunk_size);
        println!("ICFL_INDEXES={:?}", icfl_indexes);
        println!("ICFL FACTORS: {:?}", factors);
        println!("CSTM_INDEXES={:?}", custom_indexes);
        println!("CSTM FACTORS: {:?}", custom_factors);
        println!("is_custom_vec={:?}", is_custom_vec);
        println!("factor_list={:?}", factor_list);
    }

    // Prefix Trie Structure create
    let mut root = create_prefix_trie(str, src_length, &custom_indexes, &is_custom_vec);

    if debug_verbose {
        println!("Before merge");
        root.print(0, "".into());
    }

    // Merge Rankings (Canonical and Custom)
    let mut wbsa = (0..src_length).collect::<Vec<_>>();
    root.merge_rankings_and_sort_recursive(str, &mut wbsa, 0);

    if debug_verbose {
        println!("Before SHRINK");
        root.print_with_wbsa(0, "".into(), &wbsa);
    }

    root.shrink_bottom_up(&mut wbsa, str, &icfl_indexes, &is_custom_vec, &factor_list);

    if debug_verbose {
        println!("After SHRINK");
        root.print_with_wbsa(0, "".into(), &wbsa);
        println!("{:?}", wbsa);
    }

    let after = Instant::now();

    // println!("Total time: {}", duration.as_secs_f32());

    InnovativeSuffixArrayComputationResults {
        suffix_array: wbsa,
        duration: after - before,
    }
}
