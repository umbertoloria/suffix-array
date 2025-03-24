use crate::suffix_array::chunking::get_max_factor_size;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

const PREFIX_TRIE_FIRST_IDS_START_FROM: usize = 0;
pub fn create_prefix_trie(
    src: &str,
    src_length: usize,
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    depths: &mut Vec<usize>,
    monitor: &mut Monitor,
    verbose: bool,
) -> PrefixTrie {
    let max_factor_size =
        get_max_factor_size(&custom_indexes, src_length).expect("max_factor_size is not valid");

    let mut next_index = PREFIX_TRIE_FIRST_IDS_START_FROM;
    let mut root = PrefixTrie::new(next_index, 0);
    next_index += 1;

    let custom_indexes_len = custom_indexes.len();
    let last_factor_size = src_length - custom_indexes[custom_indexes_len - 1];

    for curr_ls_size in 1..max_factor_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "curr_suffix_length" and,
        // if there exist, takes their Local Suffixes of "curr_suffix_length" length.

        // Last Factor
        if curr_ls_size <= last_factor_size {
            let ls_index = src_length - curr_ls_size;
            let is_custom_ls = is_custom_vec[ls_index];
            root.add_string(
                &mut next_index,
                ls_index,
                curr_ls_size,
                src,
                is_custom_ls,
                verbose,
            );
            depths[ls_index] = curr_ls_size;
            if verbose {
                root.print(0, "".into());
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
                    src,
                    is_custom_ls,
                    verbose,
                );
                depths[ls_index] = curr_ls_size;
                if verbose {
                    root.print(0, "".into());
                }
            }
        }
    }

    root
}

// TODO: Maybe understand when to adjust "MIN_SIZE_DIRECT_CHILD_SUBSTRING" (if needed)
const MIN_SIZE_DIRECT_CHILD_SUBSTRING: usize = 2;
pub struct PrefixTrie {
    pub id: usize,
    pub suffix_len: usize,
    pub data: PrefixTrieData,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    pub rankings_final: Vec<usize>,
}
pub enum PrefixTrieData {
    DirectChild((String, Box<PrefixTrie>)),
    Children(BTreeMap<char, PrefixTrie>),
    Leaf,
    InitRoot, // Will be replaced with "Children" as soon as First Layer Nodes come in.
}
impl PrefixTrie {
    pub fn new(id: usize, suffix_len: usize) -> Self {
        Self {
            id,
            suffix_len,
            data: if suffix_len == 0 {
                PrefixTrieData::InitRoot
            } else {
                PrefixTrieData::Leaf
            },
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

    // Prints
    pub fn print(&self, tabs_offset: usize, prefix_rec: String) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            prefix_rec,
            format!("{:?} {:?}", self.rankings_canonical, self.rankings_custom),
        );
        match &self.data {
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    child_node.print(tabs_offset + 1, format!("{}{}", prefix_rec, char_key));
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                child_node.print(tabs_offset + 1, format!("{}{}", prefix_rec, prefix));
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }
    }
    pub fn print_merged(&self, tabs_offset: usize, prefix_rec: String) {
        println!(
            "{}\"{}\" {:?}",
            "\t".repeat(tabs_offset),
            prefix_rec,
            self.rankings_final,
        );
        match &self.data {
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    child_node.print_merged(tabs_offset + 1, format!("{}{}", prefix_rec, char_key));
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                child_node.print_merged(tabs_offset + 1, format!("{}{}", prefix_rec, prefix));
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }
    }

    // Tree transformation
    fn add_string(
        &mut self,
        next_index: &mut usize,
        ls_index: usize,
        ls_size: usize,
        str: &str,
        is_custom_ls: bool,
        verbose: bool,
    ) {
        if verbose {
            println!(
                "{}add_string > ls_index={ls_index}, ls_size={ls_size}, self.suffix_len={}, word={:?}",
                "  ".repeat(self.suffix_len),
                self.suffix_len,
                &str[ls_index..ls_index + ls_size]
            );
        }

        let i_letter_ls = self.suffix_len;
        if i_letter_ls >= ls_size {
            if i_letter_ls > ls_size {
                // Should never happen...
                // exit(0x0100);
            } else {
                if verbose {
                    println!(
                        "{}  > found, only update rankings",
                        "  ".repeat(self.suffix_len)
                    );
                }

                self.update_rankings(ls_index, is_custom_ls);
            }
            return;
        }

        // From here on: "i_letter_ls < ls_size"
        let curr_letter = (&str[ls_index + i_letter_ls..ls_index + i_letter_ls + 1])
            .chars()
            .next()
            .unwrap();

        match &mut self.data {
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let rest_of_ls = &str[ls_index + i_letter_ls..ls_index + ls_size];
                if rest_of_ls == prefix {
                    child_node.update_rankings(ls_index, is_custom_ls);

                    return;
                }

                if verbose {
                    println!(
                        "{}  > create regular child (after normalizing direct child node)",
                        "  ".repeat(self.suffix_len)
                    );
                }

                let prefix_first_letter = (&prefix[0..1]).chars().next().unwrap();

                // Node "child_node" will disappear, so its ID will be used by "new_child_node"
                let mut new_child_node = PrefixTrie::new(child_node.id, self.suffix_len + 1);
                for &ranking_canonical in &child_node.rankings_canonical {
                    new_child_node.add_string(
                        next_index,
                        ranking_canonical,
                        self.suffix_len + prefix.len(),
                        str,
                        false,
                        verbose,
                    );
                }
                for &ranking_custom in &child_node.rankings_custom {
                    new_child_node.add_string(
                        next_index,
                        ranking_custom,
                        self.suffix_len + prefix.len(),
                        str,
                        true,
                        verbose,
                    );
                }

                if verbose {
                    println!(
                        "{}     (setting on {})",
                        "  ".repeat(self.suffix_len),
                        prefix_first_letter
                    );
                }

                let mut children = BTreeMap::new();
                children.insert(prefix_first_letter, new_child_node);
                self.data = PrefixTrieData::Children(children);

                // Re-try now that the Direct Child Node has been normalized (De-Directed).
                self.add_string(next_index, ls_index, ls_size, str, is_custom_ls, verbose);
            }
            PrefixTrieData::Children(children) => {
                if children.contains_key(&curr_letter) {
                    if verbose {
                        println!(
                            "{}  > contained {}",
                            "  ".repeat(self.suffix_len),
                            curr_letter
                        );
                    }

                    let child_node = children.get_mut(&curr_letter).unwrap();
                    child_node.add_string(
                        next_index,
                        ls_index,
                        ls_size,
                        str,
                        is_custom_ls,
                        verbose,
                    );
                } else {
                    if verbose {
                        println!("{}  > create regular child", "  ".repeat(self.suffix_len));
                    }

                    let mut new_child_node = PrefixTrie::new(*next_index, self.suffix_len + 1);
                    *next_index += 1;
                    new_child_node.add_string(
                        next_index,
                        ls_index,
                        ls_size,
                        str,
                        is_custom_ls,
                        verbose,
                    );

                    children.insert(curr_letter, new_child_node);
                }
            }
            PrefixTrieData::Leaf => {
                // Assuming "self.suffix_len > 0".

                if ls_size - i_letter_ls >= MIN_SIZE_DIRECT_CHILD_SUBSTRING {
                    // Here we are in a leaf. So we create a Direct Child Node instead of a Path
                    // made of multiple Child Nodes.

                    let rest_of_ls = &str[ls_index + i_letter_ls..ls_index + ls_size];

                    if verbose {
                        println!(
                            "{}  > create direct child \"{}\"",
                            "  ".repeat(self.suffix_len),
                            rest_of_ls
                        );
                    }

                    // This is the first inserted Child Node.
                    let mut new_child_node = PrefixTrie::new(*next_index, ls_size);
                    *next_index += 1;
                    new_child_node.update_rankings(ls_index, is_custom_ls);

                    self.data = PrefixTrieData::DirectChild((
                        //
                        rest_of_ls.to_string(),
                        Box::new(new_child_node),
                    ));
                } else {
                    if verbose {
                        println!("{}  > create regular child", "  ".repeat(self.suffix_len));
                    }

                    // This is the first inserted Child Node.
                    let mut new_child_node = PrefixTrie::new(*next_index, self.suffix_len + 1);
                    *next_index += 1;
                    new_child_node.add_string(
                        next_index,
                        ls_index,
                        ls_size,
                        str,
                        is_custom_ls,
                        verbose,
                    );

                    let mut children = BTreeMap::new();
                    children.insert(curr_letter, new_child_node);
                    self.data = PrefixTrieData::Children(children);
                }
            }
            PrefixTrieData::InitRoot => {
                // This will become a (Root) Node with Children.
                if verbose {
                    println!("{}  > create regular child", "  ".repeat(self.suffix_len));
                }

                // This is the first inserted Child Node.
                let mut new_child_node = PrefixTrie::new(*next_index, self.suffix_len + 1);
                *next_index += 1;
                new_child_node.add_string(
                    next_index,
                    ls_index,
                    ls_size,
                    str,
                    is_custom_ls,
                    verbose,
                );

                let mut children = BTreeMap::new();
                children.insert(curr_letter, new_child_node);
                self.data = PrefixTrieData::Children(children);
            }
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
        match &mut self.data {
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    child_node.merge_rankings_and_sort_recursive(str);
                    /*
                    let new_p = child_node.merge_rankings_and_sort_recursive(str, wbsa, wbsa_indexes, p);
                    p = new_p;
                    */
                }
            }
            PrefixTrieData::DirectChild((_, child_node)) => {
                child_node.merge_rankings_and_sort_recursive(str);
                /*
                let new_p = child_node.merge_rankings_and_sort_recursive(str, wbsa, wbsa_indexes, p);
                p = new_p;
                */
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }

        // p
    }
}

// PREFIX TRIE LOGGER
pub fn log_prefix_trie(root: &PrefixTrie, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    match &root.data {
        PrefixTrieData::Children(children) => {
            for (char_key, child_node) in children {
                let child_label = &format!("{}", char_key);
                log_prefix_trie_recursive(child_node, child_label, &mut file, 0);
            }
        }
        PrefixTrieData::DirectChild((prefix, child_node)) => {
            let child_label = &format!("{}", prefix);
            log_prefix_trie_recursive(child_node, child_label, &mut file, 0);
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
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
    match &node.data {
        PrefixTrieData::Children(children) => {
            for (char_key, child_node) in children {
                let child_label = &format!("{}{}", node_label, char_key);
                log_prefix_trie_recursive(child_node, child_label, file, level + 1);
            }
        }
        PrefixTrieData::DirectChild((prefix, child_node)) => {
            let child_label = &format!("{}{}", node_label, prefix);
            // log_prefix_trie_recursive(child_node, child_label, file, level + 1);
            log_prefix_trie_recursive(child_node, child_label, file, level + prefix.len());
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
    }
}
