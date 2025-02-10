use crate::suffix_array::chunking::get_max_size;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::collections::{BTreeMap, HashMap};

pub type WbsaIndexes = HashMap<usize, (usize, usize)>;
pub fn create_prefix_trie(
    src: &str,
    src_length: usize,
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    wbsa_indexes: &mut WbsaIndexes,
    depths: &mut Vec<usize>,
    monitor: &mut Monitor,
) -> PrefixTrie {
    let custom_max_size =
        get_max_size(&custom_indexes, src_length).expect("custom_max_size is not valid");

    let mut root = PrefixTrie::new(0, 0, wbsa_indexes);
    let mut next_node_index = 1;

    let custom_indexes_len = custom_indexes.len();
    let last_custom_factor_index = custom_indexes[custom_indexes_len - 1];
    let last_custom_factor_size = src_length - last_custom_factor_index;

    for curr_suffix_length in 1..custom_max_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "curr_suffix_length" and,
        // if there exist, takes their Local Suffixes of "curr_suffix_length" length.

        // Last Custom Factor
        if curr_suffix_length <= last_custom_factor_size {
            let custom_factor_local_suffix_index = src_length - curr_suffix_length;
            add_node_to_prefix_trie(
                src,
                is_custom_vec,
                &mut root,
                curr_suffix_length,
                custom_factor_local_suffix_index,
                wbsa_indexes,
                depths,
                &mut next_node_index,
                monitor,
            );
        }

        // All Custom Factors from first to second-last
        for i_custom_factor in 0..custom_indexes_len - 1 {
            let curr_custom_factor_size =
                custom_indexes[i_custom_factor + 1] - custom_indexes[i_custom_factor];
            if curr_suffix_length <= curr_custom_factor_size {
                let custom_factor_local_suffix_index =
                    custom_indexes[i_custom_factor + 1] - curr_suffix_length;
                add_node_to_prefix_trie(
                    src,
                    is_custom_vec,
                    &mut root,
                    curr_suffix_length,
                    custom_factor_local_suffix_index,
                    wbsa_indexes,
                    depths,
                    &mut next_node_index,
                    monitor,
                );
            }
        }
    }

    root
}

fn add_node_to_prefix_trie(
    src: &str,
    is_custom_vec: &Vec<bool>,
    root: &mut PrefixTrie,
    curr_suffix_length: usize,
    custom_factor_local_suffix_index: usize,
    wbsa_indexes: &mut WbsaIndexes,
    depths: &mut Vec<usize>,
    next_node_index: &mut usize,
    monitor: &mut Monitor,
) {
    let local_suffix = &src
        [custom_factor_local_suffix_index..custom_factor_local_suffix_index + curr_suffix_length];
    let chars_local_suffix = local_suffix.chars().collect::<Vec<_>>();

    let mut curr_node = root;

    let mut i_chars_of_suffix = 0; // This is the current "depth" of "curr_node".
    while i_chars_of_suffix < curr_suffix_length {
        let curr_letter = chars_local_suffix[i_chars_of_suffix];

        // Remember: using "curr_node.sons.entry(curr_letter).or_insert(" is slower.
        if !curr_node.sons.contains_key(&curr_letter) {
            curr_node.sons.insert(
                curr_letter,
                PrefixTrie::new(*next_node_index, i_chars_of_suffix + 1, wbsa_indexes),
            );
            *next_node_index += 1;
        }
        curr_node = curr_node.sons.get_mut(&curr_letter).unwrap();

        i_chars_of_suffix += 1;
    }
    if is_custom_vec[custom_factor_local_suffix_index] {
        curr_node
            .rankings_custom
            .push(custom_factor_local_suffix_index);
    } else {
        curr_node
            .rankings_canonical
            .push(custom_factor_local_suffix_index);
    }
    depths[custom_factor_local_suffix_index] = curr_node.suffix_len;
}

pub struct PrefixTrie {
    pub index: usize,
    pub suffix_len: usize,
    // TODO: Try to use HashMap but keeping chars sorted
    pub sons: BTreeMap<char, PrefixTrie>,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    pub wbsa_p: usize, // Incl.
    pub wbsa_q: usize, // Excl.
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTrie {
    // Constructor
    pub fn new(index: usize, suffix_len: usize, wbsa_indexes: &mut WbsaIndexes) -> Self {
        wbsa_indexes.insert(index, (0, 0));
        Self {
            index,
            suffix_len,
            sons: BTreeMap::new(),
            rankings_canonical: Vec::new(),
            rankings_custom: Vec::new(),
            wbsa_p: 0,
            wbsa_q: 0,
            min_father: None,
            max_father: None,
        }
    }
    // Getters
    fn get_buff_index_left(&self, wbsa_indexes: &WbsaIndexes) -> usize {
        wbsa_indexes.get(&self.index).unwrap().0
        // self.wbsa_p // TODO: Remove "wbsa_p" and "wbsa_q"?
    }
    fn get_buff_index_right_excl(&self, wbsa_indexes: &WbsaIndexes) -> usize {
        wbsa_indexes.get(&self.index).unwrap().1
        // self.wbsa_q
    }
    fn get_rankings_count(&self, wbsa_indexes: &WbsaIndexes) -> usize {
        self.get_buff_index_right_excl(wbsa_indexes) - self.get_buff_index_left(wbsa_indexes)
    }
    fn get_max_buff_index_right_excl_from_righted_child(
        &self,
        wbsa_indexes: &WbsaIndexes,
    ) -> usize {
        if self.sons.is_empty() {
            self.get_buff_index_right_excl(wbsa_indexes)
        } else {
            let sons = &self.sons.values().collect::<Vec<_>>();
            let last_son = sons[sons.len() - 1];
            last_son.get_max_buff_index_right_excl_from_righted_child(wbsa_indexes)
        }
        /*
        // TODO: Maybe this code is faster
        let mut oracle = 0;
        if self.sons.is_empty() {
            oracle = self.get_buff_index_right_excl()
        } else {
            let sons = &self.sons.values().collect::<Vec<_>>();
            let last_son = sons[sons.len() - 1];
            oracle = last_son.get_max_buff_index_right_excl_from_righted_child()
        }
        let mut test = 0;
        if let Some((_, last_son)) = self.sons.last_key_value() {
            test = last_son.get_max_buff_index_right_excl_from_righted_child()
        } else {
            test = self.get_buff_index_right_excl()
        }
        if oracle == test {
            println!("yes: {} and {}", oracle, test);
        } else {
            println!("**************** should be {} but is {}", oracle, test);
        }
        oracle
        */
    }
    fn get_first_ls_index(&self, wbsa: &Vec<usize>, wbsa_indexes: &WbsaIndexes) -> usize {
        wbsa[self.get_buff_index_left(wbsa_indexes)]
    }
    pub fn get_rankings<'a>(
        &self,
        wbsa: &'a Vec<usize>,
        wbsa_indexes: &WbsaIndexes,
    ) -> &'a [usize] {
        &wbsa[self.get_buff_index_left(wbsa_indexes)..self.get_buff_index_right_excl(wbsa_indexes)]
    }
    /*
    fn get_string_from_first_ranking_with_length<'a>(
        &self,
        wbsa: &Vec<usize>,
        str: &'a str,
        string_length: usize,
    ) -> &'a str {
        let child_ls_index = self.get_first_ls_index(wbsa);
        &str[child_ls_index..child_ls_index + string_length]
    }
    */

    // Prints
    pub fn print(&self, tabs_offset: usize, prefix: String) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            prefix,
            format!("{:?} {:?}", self.rankings_canonical, self.rankings_custom),
        );
        for (char_key, node) in &self.sons {
            node.print(tabs_offset + 1, format!("{}{}", prefix, char_key));
        }
    }
    pub fn print_with_wbsa(
        &self,
        tabs_offset: usize,
        prefix: String,
        wbsa: &Vec<usize>,
        wbsa_indexes: &WbsaIndexes,
    ) {
        println!(
            "{}\"{}\" {:?}   min={}, MAX={}",
            "\t".repeat(tabs_offset),
            prefix,
            self.get_rankings(wbsa, wbsa_indexes),
            if let Some(x) = self.min_father {
                format!("{}", x)
            } else {
                "-1".into()
            },
            if let Some(x) = self.max_father {
                format!("{}", x)
            } else {
                "-1".into()
            },
        );
        for (char_key, node) in &self.sons {
            node.print_with_wbsa(
                tabs_offset + 1,
                format!("{}{}", prefix, char_key),
                wbsa,
                wbsa_indexes,
            );
        }
    }

    // Tree transformation
    pub fn merge_rankings_and_sort_recursive(
        &mut self,
        str: &str,
        wbsa: &mut Vec<usize>,
        wbsa_indexes: &mut WbsaIndexes,
        wbsa_start_from_index: usize,
    ) -> usize {
        // Here we sort the Rankings Custom (all real Global Suffixes) and then try to merge the
        // two lists Rankings Canonical Rankings Custom (Sorted) by doing a pair-comparison.
        // We don't sort Rankings Canonical because that list already contains Global Suffixes in
        // the right order (unlike Ranking Custom, that we have to sort).
        let mut sorted_rankings_custom = Vec::new();
        if !self.rankings_custom.is_empty() {
            let mut sorted_rankings_custom_pairs_list = Vec::new();
            for &local_suffix_index in &self.rankings_custom {
                sorted_rankings_custom_pairs_list
                    .push((local_suffix_index, &str[local_suffix_index..]));
            }
            // TODO: Monitor string compare
            sort_pair_vector_of_indexed_strings(&mut sorted_rankings_custom_pairs_list);
            for (custom_gs_index, _) in sorted_rankings_custom_pairs_list {
                sorted_rankings_custom.push(custom_gs_index);
            }
        }
        // OK, now Rankings Customs is sorted as well. Rankings Canonical was already sorted. Now we
        // perform the merge between these lists.
        let mut unified_rankings = Vec::new();
        let mut i_canonical = 0;
        let mut i_custom = 0;
        while i_canonical < self.rankings_canonical.len() && i_custom < sorted_rankings_custom.len()
        {
            let canonical_gs_index = self.rankings_canonical[i_canonical];
            let canonical_gs = &str[canonical_gs_index..];

            let custom_gs_index = sorted_rankings_custom[i_custom];
            let custom_gs = &str[custom_gs_index..];

            if canonical_gs < custom_gs {
                unified_rankings.push(canonical_gs_index);
                i_canonical += 1;
            } else {
                // Case "equals" should never happen.
                unified_rankings.push(custom_gs_index);
                i_custom += 1;
            }
        }
        while i_canonical < self.rankings_canonical.len() {
            let canonical_gs_index = self.rankings_canonical[i_canonical];
            unified_rankings.push(canonical_gs_index);
            i_canonical += 1;
        }
        while i_custom < sorted_rankings_custom.len() {
            let custom_gs_index = sorted_rankings_custom[i_custom];
            unified_rankings.push(custom_gs_index);
            i_custom += 1;
        }

        let mut p = wbsa_start_from_index;
        self.wbsa_p = p; // TODO: Useless to update "self.wbsa_p"
        let bkp_p = p;
        if !unified_rankings.is_empty() {
            // Update list only if strings were actually sorted and moved.
            for index in unified_rankings {
                wbsa[p] = index;
                p += 1;
            }
        }
        self.wbsa_q = p; // TODO: Useless to update "self.wbsa_q"
        wbsa_indexes.insert(self.index, (bkp_p, p));

        // Recursive calls...
        for (_, son) in &mut self.sons {
            let new_p = son.merge_rankings_and_sort_recursive(str, wbsa, wbsa_indexes, p);
            p = new_p;
        }

        p
    }
}
