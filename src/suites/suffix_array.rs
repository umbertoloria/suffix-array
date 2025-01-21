use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use crate::suffix_array::chunking::{
    get_custom_factor_strings_from_custom_indexes, get_custom_factors, get_factor_list,
    get_indexes_from_factors, get_is_custom_vec,
};
use crate::suffix_array::prefix_trie::create_prefix_trie;
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;

pub fn main_suffix_array() {
    let src = get_fasta_content("generated/001.fasta".into());
    let src_str = src.as_str();
    let src_length = src.len();
    println!("STRING={}", src_str);
    compute_classic_suffix_array(src_str);

    // Compute ICFL
    let factors = icfl(src_str);

    let chunk_size = 3;
    println!("chunk_size={}", chunk_size);
    // TODO: Simplify algorithms by having string length as last item of these Factor Index vectors
    let icfl_indexes = get_indexes_from_factors(&factors);
    println!("ICFL_INDEXES={:?}", icfl_indexes);
    println!("ICFL FACTORS: {:?}", factors);

    let custom_indexes = get_custom_factors(&icfl_indexes, chunk_size, src_length);
    let custom_factors = get_custom_factor_strings_from_custom_indexes(src_str, &custom_indexes);
    println!("CSTM_INDEXES={:?}", custom_indexes);
    println!("CSTM FACTORS: {:?}", custom_factors);

    // let max_size = get_max_size(&icfl_indexes, src_length).expect("max_size is not valid");
    // println!("MAX_SIZE={:?}", max_size);
    // println!("CSTM_MAX_SIZE={:?}", custom_max_size);

    // TODO: Optimize both functions and rename them (and variables)
    // Factor List: [Source Char Index] => True if it's part of the last Custom Factor of an
    //                                     ICFL Factor, so it's a Local Suffix
    let is_custom_vec = get_is_custom_vec(&icfl_indexes, src_length, chunk_size);
    println!("is_custom_vec={:?}", is_custom_vec);

    // Factor List: [Source Char Index] => ICFL Factor Index of that
    let factor_list = get_factor_list(&icfl_indexes, src_length);
    println!("factor_list={:?}", factor_list);

    // Prefix Trie Structure create
    let mut root = create_prefix_trie(&src, src_length, &custom_indexes, &is_custom_vec);

    // Ordering rankings.
    println!("Before merge");
    root.print(0, "".into());

    // Merge Rankings (Canonical and Custom)
    let mut wbsa = (0..src_length).collect::<Vec<_>>();
    root.merge_rankings_and_sort_recursive(src_str, &mut wbsa, 0);
    println!("WANNA BE SUFFIX ARRAY: {:?}", &wbsa);

    println!("Before in_prefix");
    root.print_with_wbsa(0, "".into(), &wbsa);

    // In Prefix Merge: bit vector
    /*root.in_prefix_merge_bit_vector(src_str, &icfl_indexes, &is_custom_vec, &factor_list);
    root.print(0, "".into());*/

    root.shrink_bottom_up(&mut wbsa);
}

fn compute_classic_suffix_array(src: &str) {
    let mut suffix_array = Vec::new();
    for i in 0..src.len() {
        suffix_array.push((i, &src[i..]));
    }
    sort_pair_vector_of_indexed_strings(&mut suffix_array);
    for (index, suffix) in suffix_array {
        println!(" > SUFFIX_ARRAY [{:3}] = {}", index, suffix);
    }
}
