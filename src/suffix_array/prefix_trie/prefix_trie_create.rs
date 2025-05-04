use crate::suffix_array::chunking::get_max_factor_size;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::new_tree::Tree;
use crate::suffix_array::prefix_trie::prefix_trie::PrefixTrie;

pub fn create_prefix_trie<'a>(
    s_bytes: &'a [u8],
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    depths: &mut Vec<usize>,
    monitor: &mut Monitor,
    verbose: bool,
    str: &str,
) -> (PrefixTrie<'a>, Tree) {
    let str_length = s_bytes.len();
    let max_factor_size =
        get_max_factor_size(&custom_indexes, str_length).expect("max_factor_size is not valid");
    let mut next_index_src = 0;
    let next_index = &mut next_index_src;
    let mut root = PrefixTrie::new(next_index, 0);

    let mut tree = Tree::new();

    let custom_indexes_len = custom_indexes.len();
    let last_factor_size = str_length - custom_indexes[custom_indexes_len - 1];

    let mut params_canonical = Vec::new();
    let mut params_custom = Vec::new();

    for ls_size in 1..max_factor_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "ls_size" and, if there
        // exist, takes their Local Suffixes of "ls_size" length.

        // Last Factor
        if ls_size <= last_factor_size {
            let ls_index = str_length - ls_size;
            let is_custom_ls = is_custom_vec[ls_index];
            if is_custom_ls {
                params_custom.push((ls_index, ls_size));
            } else {
                params_canonical.push((ls_index, ls_size));
            }
        }

        // All Factors from first to second-last
        for i_factor in 0..custom_indexes_len - 1 {
            let curr_factor_size = custom_indexes[i_factor + 1] - custom_indexes[i_factor];
            if ls_size <= curr_factor_size {
                let ls_index = custom_indexes[i_factor + 1] - ls_size;
                let is_custom_ls = is_custom_vec[ls_index];
                if is_custom_ls {
                    params_custom.push((ls_index, ls_size));
                } else {
                    params_canonical.push((ls_index, ls_size));
                }
            }
        }
    }

    // LSs that come from Canonical Factors (already sorted)
    for (ls_index, ls_size) in params_canonical {
        tree.add(ls_index, ls_size, false, s_bytes);

        root.add_string(
            ls_index,
            ls_size,
            false,
            next_index,
            s_bytes,
            is_custom_vec,
            verbose,
        );
        depths[ls_index] = ls_size;
        if verbose {
            root.print_before_shrink(0, "", str);
        }
    }

    // LSs that come from Custom Factors (to sort)
    for (ls_index, ls_size) in params_custom {
        tree.add(ls_index, ls_size, true, s_bytes);

        root.add_string(
            ls_index,
            ls_size,
            true,
            next_index,
            s_bytes,
            is_custom_vec,
            verbose,
        );
        depths[ls_index] = ls_size;
        if verbose {
            root.print_before_shrink(0, "", str);
        }
    }

    (root, tree)
}
