use crate::suffix_array::chunking::get_max_factor_size;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::collections::btree_map::{Iter, IterMut};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

pub fn create_prefix_trie(
    src: &str,
    src_length: usize,
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    depths: &mut Vec<usize>,
    monitor: &mut Monitor,
) -> PrefixTrie {
    let max_factor_size =
        get_max_factor_size(&custom_indexes, src_length).expect("max_factor_size is not valid");

    let mut root = PrefixTrie::new(0);

    let custom_indexes_len = custom_indexes.len();
    let last_factor_size = src_length - custom_indexes[custom_indexes_len - 1];

    for curr_ls_size in 1..max_factor_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "curr_suffix_length" and,
        // if there exist, takes their Local Suffixes of "curr_suffix_length" length.

        // Last Factor
        if curr_ls_size <= last_factor_size {
            let ls_index = src_length - curr_ls_size;
            let is_custom_ls = is_custom_vec[ls_index];
            root.add_string(ls_index, curr_ls_size, src, is_custom_ls);
            depths[ls_index] = curr_ls_size;
        }

        // All Factors from first to second-last
        for i_factor in 0..custom_indexes_len - 1 {
            let curr_factor_size = custom_indexes[i_factor + 1] - custom_indexes[i_factor];
            if curr_ls_size <= curr_factor_size {
                let ls_index = custom_indexes[i_factor + 1] - curr_ls_size;
                let is_custom_ls = is_custom_vec[ls_index];
                root.add_string(ls_index, curr_ls_size, src, is_custom_ls);
                depths[ls_index] = curr_ls_size;
            }
        }
    }

    root
}

pub struct PrefixTrie {
    pub suffix_len: usize,
    // TODO: Try to use HashMap but keeping chars sorted
    pub sons: BTreeMap<char, PrefixTrie>,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    pub rankings_final: Vec<usize>,
}
pub enum PrefixTrieChildren<'a> {
    ManyChildren(Iter<'a, char, PrefixTrie>),
}
pub enum PrefixTrieChildrenMut<'a> {
    ManyChildren(IterMut<'a, char, PrefixTrie>),
}

impl PrefixTrie {
    pub fn new(suffix_len: usize) -> Self {
        Self {
            suffix_len,
            sons: BTreeMap::new(),
            rankings_canonical: Vec::new(),
            rankings_custom: Vec::new(),
            rankings_final: Vec::new(),
        }
    }
    // Getters
    /*
    fn get_buff_index_left(&self, wbsa_indexes: &WbsaIndexes) -> usize {
        wbsa_indexes.get(&self.index).unwrap().0
        // self.wbsa_p
    }
    fn get_buff_index_right_excl(&self, wbsa_indexes: &WbsaIndexes) -> usize {
        wbsa_indexes.get(&self.index).unwrap().1
        // self.wbsa_q
    }
    pub fn get_rankings<'a>(
        &self,
        wbsa: &'a Vec<usize>,
        wbsa_indexes: &WbsaIndexes,
    ) -> &'a [usize] {
        &wbsa[self.get_buff_index_left(wbsa_indexes)..self.get_buff_index_right_excl(wbsa_indexes)]
    }
    */
    /*
    pub fn get_rankings(&self) -> &Vec<usize> {
        &self.rankings_final
    }
    */

    pub fn get_children(&self) -> PrefixTrieChildren {
        PrefixTrieChildren::ManyChildren(self.sons.iter())
    }
    fn get_children_mut(&mut self) -> PrefixTrieChildrenMut {
        PrefixTrieChildrenMut::ManyChildren(self.sons.iter_mut())
    }

    // Prints
    pub fn print(&self, tabs_offset: usize, prefix: String) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            prefix,
            format!("{:?} {:?}", self.rankings_canonical, self.rankings_custom),
        );
        match self.get_children() {
            PrefixTrieChildren::ManyChildren(children) => {
                for (char_key, child_node) in children {
                    child_node.print(tabs_offset + 1, format!("{}{}", prefix, char_key));
                }
            }
        }
    }
    pub fn print_merged(&self, tabs_offset: usize, prefix: String) {
        println!(
            "{}\"{}\" {:?}",
            "\t".repeat(tabs_offset),
            prefix,
            self.rankings_final,
        );
        match self.get_children() {
            PrefixTrieChildren::ManyChildren(children) => {
                for (char_key, child_node) in children {
                    child_node.print_merged(tabs_offset + 1, format!("{}{}", prefix, char_key));
                }
            }
        }
    }

    // Tree transformation
    fn add_string(&mut self, ls_index: usize, ls_size: usize, str: &str, is_custom_ls: bool) {
        let i_letter_ls = self.suffix_len;
        if i_letter_ls < ls_size {
            let curr_letter = (
                //
                &str[ls_index + i_letter_ls..ls_index + i_letter_ls + 1]
            )
                .chars()
                .next()
                .unwrap();

            if self.sons.contains_key(&curr_letter) {
                let child_node = self.sons.get_mut(&curr_letter).unwrap();
                child_node.add_string(ls_index, ls_size, str, is_custom_ls);

                return;
            }

            let mut child_node = PrefixTrie::new(i_letter_ls + 1);
            child_node.add_string(ls_index, ls_size, str, is_custom_ls);

            self.sons.insert(curr_letter, child_node);
        } else if i_letter_ls == ls_size {
            self.update_rankings(ls_index, is_custom_ls);
        } else {
            // Should never happen...
        }
    }
    fn update_rankings(&mut self, ls_index: usize, is_custom_ls: bool) {
        if is_custom_ls {
            self.rankings_custom.push(ls_index);
        } else {
            self.rankings_canonical.push(ls_index);
        }
    }
    pub fn merge_rankings_and_sort_recursive(&mut self, str: &str) {
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
        let mut unified_rankings = &mut self.rankings_final;
        unified_rankings.reserve(self.rankings_canonical.len() + sorted_rankings_custom.len());
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

        /*
        let mut p = wbsa_start_from_index;
        // self.wbsa_p = p;
        let bkp_p = p;
        if !unified_rankings.is_empty() {
            // Update list only if strings were actually sorted and moved.
            for index in unified_rankings {
                wbsa[p] = index;
                p += 1;
            }
        }
        // self.wbsa_q = p;
        wbsa_indexes.insert(self.index, (bkp_p, p));
        */

        // Recursive calls...
        match self.get_children_mut() {
            PrefixTrieChildrenMut::ManyChildren(children) => {
                for (_, child_node) in children {
                    child_node.merge_rankings_and_sort_recursive(str);
                    /*
                    let new_p = child_node.merge_rankings_and_sort_recursive(str, wbsa, wbsa_indexes, p);
                    p = new_p;
                    */
                }
            }
        }

        // p
    }
}

// PREFIX TRIE LOGGER
pub fn log_prefix_trie(root: &PrefixTrie, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    match root.get_children() {
        PrefixTrieChildren::ManyChildren(children) => {
            for (char_key, child_node) in children {
                let child_label = &format!("{}", char_key);
                log_prefix_trie_recursive(child_node, child_label, &mut file, 0);
            }
        }
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_trie_recursive(node: &PrefixTrie, node_label: &str, file: &mut File, level: usize) {
    let mut line = format!("{}{}", " ".repeat(level), node_label);
    let mut rankings = &node.rankings_final;
    if !rankings.is_empty() {
        line.push_str(" [");
        for i in 0..rankings.len() - 1 {
            let ranking = rankings[i];
            line.push_str(&format!("{}, ", ranking));
        }
        line.push_str(&format!("{}]", rankings[rankings.len() - 1]));
    }
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    match node.get_children() {
        PrefixTrieChildren::ManyChildren(children) => {
            for (char_key, child_node) in children {
                let child_label = &format!("{}{}", node_label, char_key);
                log_prefix_trie_recursive(child_node, child_label, file, level + 1);
            }
        }
    }
}
