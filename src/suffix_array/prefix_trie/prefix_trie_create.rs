use crate::suffix_array::chunking::get_max_factor_size;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_trie::prefix_trie::PrefixTrie;

const PREFIX_TRIE_FIRST_IDS_START_FROM: usize = 0;
pub fn create_prefix_trie<'a>(
    s_bytes: &'a [u8],
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    depths: &mut Vec<usize>,
    monitor: &mut Monitor,
    verbose: bool,
) -> PrefixTrie<'a> {
    let str_length = s_bytes.len();
    let max_factor_size =
        get_max_factor_size(&custom_indexes, str_length).expect("max_factor_size is not valid");
    let mut next_index = PREFIX_TRIE_FIRST_IDS_START_FROM;

    let mut root = PrefixTrie::new_using_next_index(
        //
        &mut next_index,
        0,
    );

    let custom_indexes_len = custom_indexes.len();
    let last_factor_size = str_length - custom_indexes[custom_indexes_len - 1];

    for curr_ls_size in 1..max_factor_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "curr_suffix_length" and,
        // if there exist, takes their Local Suffixes of "curr_suffix_length" length.

        // Last Factor
        if curr_ls_size <= last_factor_size {
            let ls_index = str_length - curr_ls_size;
            let is_custom_ls = is_custom_vec[ls_index];
            root.add_string(
                &mut next_index,
                ls_index,
                curr_ls_size,
                s_bytes,
                is_custom_ls,
                verbose,
            );
            depths[ls_index] = curr_ls_size;
            if verbose {
                root.print(0, "");
            }
        }

        // All Factors from first to second-last
        for i_factor in 0..custom_indexes_len - 1 {
            let curr_factor_size = custom_indexes[i_factor + 1] - custom_indexes[i_factor];
            if curr_ls_size <= curr_factor_size {
                let ls_index = custom_indexes[i_factor + 1] - curr_ls_size;
                let is_custom_ls = is_custom_vec[ls_index];
                root.add_string(
                    &mut next_index,
                    ls_index,
                    curr_ls_size,
                    s_bytes,
                    is_custom_ls,
                    verbose,
                );
                depths[ls_index] = curr_ls_size;
                if verbose {
                    root.print(0, "");
                }
            }
        }
    }

    root
}
