use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::collections::BTreeMap;

const MIN_SIZE_DIRECT_CHILD_SUBSTRING: usize = 2;
pub struct PrefixTrie<'a> {
    pub id: usize,
    pub suffix_len: usize,
    pub data: PrefixTrieData<'a>,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    pub rankings: Vec<usize>,
}
pub enum PrefixTrieData<'a> {
    DirectChild((&'a PrefixTrieString, Box<PrefixTrie<'a>>)),
    Children(BTreeMap<PrefixTrieChar, PrefixTrie<'a>>),
    Leaf,
    InitRoot, // Will be replaced with "Children" as soon as First Layer Nodes come in.
    Vec(Vec<PrefixTrie<'a>>),
}
impl<'a> PrefixTrie<'a> {
    pub fn new_using_next_index(next_index: &mut usize, suffix_len: usize) -> Self {
        let id = *next_index;
        *next_index += 1;
        Self::new(id, suffix_len)
    }
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
            rankings: Vec::new(),
        }
    }

    // Rankings
    pub fn get_rankings_canonical(&self) -> &Vec<usize> {
        &self.rankings_canonical
    }
    pub fn get_rankings_custom(&self) -> &Vec<usize> {
        &self.rankings_custom
    }

    // Tree transformation
    pub fn add_string(
        &mut self,
        next_index: &mut usize,
        ls_index: usize,
        ls_size: usize,
        s_bytes: &'a [u8],
        is_custom_ls: bool,
        verbose: bool,
    ) {
        if verbose {
            let ls_str = &s_bytes[ls_index..ls_index + ls_size];
            println!(
                "{}add_string > ls_index={ls_index}, ls_size={ls_size}, self.suffix_len={}, word={:?}",
                "  ".repeat(self.suffix_len),
                self.suffix_len,
                get_string_clone(&ls_str),
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
        let curr_letter: PrefixTrieChar = s_bytes[ls_index + i_letter_ls];

        match &mut self.data {
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let rest_of_ls = &s_bytes[ls_index + i_letter_ls..ls_index + ls_size];
                if compare_strings(rest_of_ls, prefix) {
                    child_node.update_rankings(ls_index, is_custom_ls);

                    return;
                }

                if verbose {
                    println!(
                        "{}  > create regular child (after normalizing direct child node)",
                        "  ".repeat(self.suffix_len)
                    );
                }

                let old_child_node_rankings_canonical = child_node.get_rankings_canonical();
                let old_child_node_rankings_custom = child_node.get_rankings_custom();

                // Node "child_node" will disappear, so its ID will be used by "new_child_node"
                let mut new_child_node = PrefixTrie::new(child_node.id, self.suffix_len + 1);
                for &ranking_canonical in old_child_node_rankings_canonical {
                    new_child_node.add_string(
                        next_index,
                        ranking_canonical,
                        self.suffix_len + prefix.len(),
                        s_bytes,
                        false,
                        verbose,
                    );
                }
                for &ranking_custom in old_child_node_rankings_custom {
                    new_child_node.add_string(
                        next_index,
                        ranking_custom,
                        self.suffix_len + prefix.len(),
                        s_bytes,
                        true,
                        verbose,
                    );
                }

                let prefix_first_letter: PrefixTrieChar = prefix[0];
                if verbose {
                    println!(
                        "{}     (setting on {})",
                        "  ".repeat(self.suffix_len),
                        get_string_char_clone(prefix_first_letter),
                    );
                }

                let mut children = BTreeMap::new();
                children.insert(prefix_first_letter, new_child_node);
                self.data = PrefixTrieData::Children(children);

                // Re-try now that the Direct Child Node has been normalized (De-Directed).
                self.add_string(
                    //
                    next_index,
                    ls_index,
                    ls_size,
                    s_bytes,
                    is_custom_ls,
                    verbose,
                );
            }
            PrefixTrieData::Children(children) => {
                if children.contains_key(&curr_letter) {
                    if verbose {
                        println!(
                            "{}  > contained {}",
                            "  ".repeat(self.suffix_len),
                            get_string_char_clone(curr_letter),
                        );
                    }

                    let child_node = children.get_mut(&curr_letter).unwrap();
                    child_node.add_string(
                        next_index,
                        ls_index,
                        ls_size,
                        s_bytes,
                        is_custom_ls,
                        verbose,
                    );
                } else {
                    if verbose {
                        println!("{}  > create regular child", "  ".repeat(self.suffix_len));
                    }

                    let mut new_child_node = PrefixTrie::new_using_next_index(
                        //
                        next_index,
                        self.suffix_len + 1,
                    );
                    new_child_node.add_string(
                        next_index,
                        ls_index,
                        ls_size,
                        s_bytes,
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

                    let rest_of_ls = &s_bytes[ls_index + i_letter_ls..ls_index + ls_size];
                    if verbose {
                        let rest_of_ls_str = get_string_clone(&rest_of_ls);
                        println!(
                            "{}  > create direct child \"{}\"",
                            "  ".repeat(self.suffix_len),
                            rest_of_ls_str,
                        );
                    }

                    // This is the first inserted Child Node.
                    let mut new_child_node = PrefixTrie::new_using_next_index(
                        //
                        next_index, ls_size,
                    );
                    new_child_node.update_rankings(ls_index, is_custom_ls);

                    // TODO: Avoid cloning
                    self.data = PrefixTrieData::DirectChild((
                        //
                        rest_of_ls,
                        Box::new(new_child_node),
                    ));
                } else {
                    if verbose {
                        println!("{}  > create regular child", "  ".repeat(self.suffix_len));
                    }

                    // This is the first inserted Child Node.
                    let mut new_child_node = PrefixTrie::new_using_next_index(
                        //
                        next_index,
                        self.suffix_len + 1,
                    );
                    new_child_node.add_string(
                        next_index,
                        ls_index,
                        ls_size,
                        s_bytes,
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
                let mut new_child_node = PrefixTrie::new_using_next_index(
                    //
                    next_index,
                    self.suffix_len + 1,
                );
                new_child_node.add_string(
                    next_index,
                    ls_index,
                    ls_size,
                    s_bytes,
                    is_custom_ls,
                    verbose,
                );

                let mut children = BTreeMap::new();
                children.insert(curr_letter, new_child_node);
                self.data = PrefixTrieData::Children(children);
            }
            PrefixTrieData::Vec(_) => {
                // This type "PrefixTrieData::Vec" is only used from the Shrink Phase and after.
                // Should never happen...
                // exit(0x0100);
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

    // SHRINK PHASE
    fn is_bridge_node(&self) -> bool {
        // Make sure to perform "shrink" before the "Merge Rankings and Sort" phase
        self.rankings_canonical.is_empty() && self.rankings_custom.is_empty()
    }
    pub fn shrink(&mut self, prefix_rec: &str) {
        match &mut self.data {
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                child_node.shrink(&format!("{}{}", prefix_rec, prefix_str));
            }
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    let prefix_str = get_string_char_clone(*char_key);
                    child_node.shrink(&format!("{}{}", prefix_rec, prefix_str));
                }
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(_) => {
                // Should never happen...
                // exit(0x0100);
            }
        }

        match &mut self.data {
            PrefixTrieData::Children(children) => {
                let mut become_vec = false;
                for (_, child_node) in &mut *children {
                    if child_node.is_bridge_node() {
                        become_vec = true;
                    }
                }
                if become_vec {
                    let mut children_list_char_key = Vec::new();
                    for (char_key, _) in &*children {
                        children_list_char_key.push(*char_key);
                    }
                    let mut children_list_child_node: Vec<PrefixTrie> = Vec::new();
                    for char_key in children_list_char_key {
                        let child_node = children.remove(&char_key).unwrap();
                        children_list_child_node.push(child_node);
                    }
                    let mut vec = Vec::new();
                    for child_node in children_list_child_node {
                        if child_node.is_bridge_node() {
                            // This is a Bridge Node, so consider directly its Children.
                            match child_node.data {
                                PrefixTrieData::DirectChild((_, child_node)) => {
                                    vec.push(*child_node);
                                }
                                PrefixTrieData::Children(children) => {
                                    vec.extend(children.into_values());
                                }
                                PrefixTrieData::Leaf => {
                                    // Should never happen...
                                    // exit(0x0100);
                                }
                                PrefixTrieData::InitRoot => {
                                    // Should never happen...
                                    // exit(0x0100);
                                }
                                PrefixTrieData::Vec(children) => {
                                    vec.extend(children);
                                }
                            }
                        } else {
                            vec.push(child_node);
                        }
                    }
                    self.data = PrefixTrieData::Vec(vec);
                }
            }
            PrefixTrieData::DirectChild((_, _)) => {
                // Should never happen...
                // exit(0x0100);
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(_) => {}
        }
    }

    // MERGE RANKINGS
    pub fn merge_rankings_and_sort_recursive(&mut self, str: &str) {
        // Here we sort the Rankings Custom (all real Global Suffixes) and then try to merge the
        // two lists Rankings Canonical Rankings Custom (Sorted) by doing a pair-comparison.
        // We don't sort Rankings Canonical because that list already contains Global Suffixes in
        // the right order (unlike Ranking Custom, that we have to sort).
        let mut sorted_rankings_custom = Vec::new();
        let mut old_rankings_custom = Vec::new();
        old_rankings_custom.append(&mut self.rankings_custom);
        if !old_rankings_custom.is_empty() {
            let mut sorted_rankings_custom_pairs_list = Vec::new();
            for local_suffix_index in old_rankings_custom {
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
        let mut sorted_rankings_canonical = Vec::new();
        sorted_rankings_canonical.append(&mut self.rankings_canonical);

        let mut unified_rankings = &mut self.rankings;
        unified_rankings.reserve(sorted_rankings_canonical.len() + sorted_rankings_custom.len());
        let mut i_canonical = 0;
        let mut i_custom = 0;
        while i_canonical < sorted_rankings_canonical.len()
            && i_custom < sorted_rankings_custom.len()
        {
            let canonical_gs_index = sorted_rankings_canonical[i_canonical];
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
        while i_canonical < sorted_rankings_canonical.len() {
            let canonical_gs_index = sorted_rankings_canonical[i_canonical];
            unified_rankings.push(canonical_gs_index);
            i_canonical += 1;
        }
        while i_custom < sorted_rankings_custom.len() {
            let custom_gs_index = sorted_rankings_custom[i_custom];
            unified_rankings.push(custom_gs_index);
            i_custom += 1;
        }

        // Recursive calls...
        match &mut self.data {
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    child_node.merge_rankings_and_sort_recursive(str);
                }
            }
            PrefixTrieData::DirectChild((_, child_node)) => {
                child_node.merge_rankings_and_sort_recursive(str);
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    child_node.merge_rankings_and_sort_recursive(str);
                }
            }
        }
    }
}

// String comparison abstractions
pub type PrefixTrieString = [u8];
pub type PrefixTrieChar = u8;
pub fn get_string_char_clone(char_type: PrefixTrieChar) -> String {
    // TODO: Needs allocation
    let vec = vec![char_type];
    unsafe { String::from_utf8_unchecked(vec) }
}
pub fn get_string_clone(str_type: &PrefixTrieString) -> String {
    // TODO: Needs cloning
    let cloned_vec = str_type.to_vec();
    String::from_utf8(cloned_vec).unwrap()
}
pub fn compare_strings(a: &PrefixTrieString, b: &PrefixTrieString) -> bool {
    if a.len() != b.len() {
        false
    } else {
        let mut i = 0;
        while i < a.len() && a[i] == b[i] {
            i += 1;
        }
        i >= a.len()
    }
}
