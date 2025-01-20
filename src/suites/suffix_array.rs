use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use crate::suffix_array::chunking::{
    get_custom_factor_strings_from_custom_indexes, get_custom_factors, get_factor_list,
    get_indexes_from_factors, get_is_custom_vec, get_max_size,
};
use crate::suffix_array::prefix_trie::PrefixTrie;
use std::collections::BTreeMap;

pub fn main_suffix_array() {
    let src = get_fasta_content("generated/001.fasta".into());
    let src_str = src.as_str();
    let src_length = src.len();

    // Compute ICFL
    let factors = icfl(src_str);

    let chunk_size = 3;
    println!("chunk_size={}", chunk_size);
    // TODO: Simplify algorithms by having string length as last item of these Factor Index vectors
    let icfl_indexes = get_indexes_from_factors(&factors);
    println!("ICFL_INDEXES={:?}", icfl_indexes);
    println!("ICFL FACTORS: {:?}", factors);

    let custom_indexes = get_custom_factors(&icfl_indexes, chunk_size, src_length);
    let custom_indexes_len = custom_indexes.len();
    let custom_indexes_last_index = custom_indexes_len - 1;
    let custom_factors = get_custom_factor_strings_from_custom_indexes(src_str, &custom_indexes);
    println!("CSTM_INDEXES={:?}", custom_indexes);
    println!("CSTM FACTORS: {:?}", custom_factors);

    // let max_size = get_max_size(&icfl_indexes, src_length).expect("max_size is not valid");
    let custom_max_size =
        get_max_size(&custom_indexes, src_length).expect("custom_max_size is not valid");
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

    let mut root = PrefixTrie::new();

    // Prefix Trie Structure create
    for curr_suffix_length in 1..custom_max_size + 1 {
        let mut ordered_list_of_custom_factor_local_suffix_index = Vec::new();
        // Last Custom Factor
        let curr_custom_factor_len = src_length - custom_indexes[custom_indexes_last_index];
        if curr_suffix_length <= curr_custom_factor_len {
            let custom_factor_local_suffix_index = src_length - curr_suffix_length;
            ordered_list_of_custom_factor_local_suffix_index.push(custom_factor_local_suffix_index);
        }
        // All Custom Factors from first to second-last
        for i_custom_factor in 0..custom_indexes_len - 1 {
            let curr_custom_factor_len =
                custom_indexes[i_custom_factor + 1] - custom_indexes[i_custom_factor];
            if curr_suffix_length <= curr_custom_factor_len {
                let custom_factor_local_suffix_index =
                    custom_indexes[i_custom_factor + 1] - curr_suffix_length;
                ordered_list_of_custom_factor_local_suffix_index
                    .push(custom_factor_local_suffix_index);
            }
        }

        // Filling "rankings_canonical" or "rankings_custom".
        for custom_factor_local_suffix_index in &ordered_list_of_custom_factor_local_suffix_index {
            // Implementation of "add_in_custom_prefix_trie".
            let custom_factor_local_suffix_index = *custom_factor_local_suffix_index;
            let suffix = &src[custom_factor_local_suffix_index
                ..custom_factor_local_suffix_index + curr_suffix_length];
            let chars_suffix = suffix.chars().collect::<Vec<_>>();

            let mut app_node = &mut root;

            let mut i_chars_of_suffix = 0;
            while i_chars_of_suffix < curr_suffix_length {
                let curr_letter = chars_suffix[i_chars_of_suffix];

                if !(*app_node).sons.contains_key(&curr_letter) {
                    (*app_node).sons.insert(
                        curr_letter,
                        PrefixTrie {
                            label: format!("{}{}", app_node.label, curr_letter),
                            sons: BTreeMap::new(),
                            rankings_canonical: Vec::new(),
                            rankings_custom: Vec::new(),
                            rankings: Vec::new(),
                            suffix_len: i_chars_of_suffix + 1,
                            min_father: None,
                            max_father: None,
                        },
                    );
                }
                app_node = app_node.sons.get_mut(&curr_letter).unwrap();

                i_chars_of_suffix += 1;
            }
            // TODO: Here we could create an interesting wrapping among real "non-bridge" nodes
            if is_custom_vec[custom_factor_local_suffix_index] {
                app_node
                    .rankings_custom
                    .push(custom_factor_local_suffix_index);
            } else {
                app_node
                    .rankings_canonical
                    .push(custom_factor_local_suffix_index);
            }
        }
    }

    // Ordering rankings.
    // println!("Before merge");
    // root.print(0, "".into());
    root.merge_rankings_and_sort_recursive(src_str, src_length);
    // println!("Before in_prefix");
    // root.print(0, "".into());

    // In Prefix Merge: bit vector
    root.in_prefix_merge_bit_vector(src_str, &icfl_indexes, &is_custom_vec, &factor_list);
    root.print(0, "".into());
}
