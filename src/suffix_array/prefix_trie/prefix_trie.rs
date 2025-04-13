use crate::suffix_array::prog_suffix_array::ProgSuffixArray;
use std::collections::BTreeMap;

const MIN_SIZE_DIRECT_CHILD_SUBSTRING: usize = 2;
pub struct PrefixTrie<'a> {
    pub id: usize,
    pub suffix_len: usize,
    pub data: PrefixTrieData<'a>,
    pub rankings: Vec<usize>, // Both Canonicals and Customs are maintained, always kept *sorted*.
}
pub enum PrefixTrieData<'a> {
    Leaf,
    DirectChild((&'a PrefixTrieString, Box<PrefixTrie<'a>>)),
    Children(BTreeMap<PrefixTrieChar, PrefixTrie<'a>>),
    Vec(Vec<PrefixTrie<'a>>),
}
impl<'a> PrefixTrie<'a> {
    pub fn new(suffix_len: usize) -> Self {
        Self {
            id: 0, // IDs not used before Merge Rankings Phase.
            suffix_len,
            data: PrefixTrieData::Leaf,
            rankings: Vec::new(),
        }
    }

    // Tree Population
    pub fn add_string(
        &mut self,
        ls_index: usize,
        ls_size: usize,
        is_custom_ls: bool,
        s_bytes: &'a [u8],
        is_custom_vec: &Vec<bool>, // This is used to distinguish a Ranking as Canonical or Custom.
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

                self.update_rankings(ls_index, is_custom_ls, s_bytes);
            }
            return;
        }

        // From here on: "i_letter_ls < ls_size"
        let curr_letter: PrefixTrieChar = s_bytes[ls_index + i_letter_ls];

        match &mut self.data {
            PrefixTrieData::Leaf => {
                // Assuming "self.suffix_len > 0".

                if self.suffix_len > 0 && ls_size - i_letter_ls >= MIN_SIZE_DIRECT_CHILD_SUBSTRING {
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
                    let mut new_child_node = PrefixTrie::new(ls_size);
                    new_child_node.update_rankings(ls_index, is_custom_ls, s_bytes);

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
                    let mut new_child_node = PrefixTrie::new(self.suffix_len + 1);
                    new_child_node.add_string(
                        //
                        ls_index,
                        ls_size,
                        is_custom_ls,
                        s_bytes,
                        is_custom_vec,
                        verbose,
                    );

                    let mut children = BTreeMap::new();
                    children.insert(curr_letter, new_child_node);
                    self.data = PrefixTrieData::Children(children);
                }
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
                        //
                        ls_index,
                        ls_size,
                        is_custom_ls,
                        s_bytes,
                        is_custom_vec,
                        verbose,
                    );
                } else {
                    if verbose {
                        println!("{}  > create regular child", "  ".repeat(self.suffix_len));
                    }

                    let mut new_child_node = PrefixTrie::new(self.suffix_len + 1);
                    new_child_node.add_string(
                        //
                        ls_index,
                        ls_size,
                        is_custom_ls,
                        s_bytes,
                        is_custom_vec,
                        verbose,
                    );

                    children.insert(curr_letter, new_child_node);
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let rest_of_ls = &s_bytes[ls_index + i_letter_ls..ls_index + ls_size];
                if compare_strings(rest_of_ls, prefix) {
                    child_node.update_rankings(ls_index, is_custom_ls, s_bytes);

                    return;
                }

                if verbose {
                    println!(
                        "{}  > create regular child (after normalizing direct child node)",
                        "  ".repeat(self.suffix_len)
                    );
                }

                // Node "child_node" will disappear, so its ID will be used by "new_child_node"
                let mut new_child_node = PrefixTrie::new(self.suffix_len + 1);

                let mut old_child_node_rankings_canonical = Vec::new();
                let mut old_child_node_rankings_custom = Vec::new();
                for &child_node_ls_index in &child_node.rankings {
                    let is_custom_ls = is_custom_vec[child_node_ls_index];
                    if is_custom_ls {
                        old_child_node_rankings_custom.push(child_node_ls_index);
                    } else {
                        old_child_node_rankings_canonical.push(child_node_ls_index);
                    }
                }

                for ranking_canonical in old_child_node_rankings_canonical {
                    new_child_node.add_string(
                        //
                        ranking_canonical,
                        self.suffix_len + prefix.len(),
                        false,
                        s_bytes,
                        is_custom_vec,
                        verbose,
                    );
                }
                for ranking_custom in old_child_node_rankings_custom {
                    new_child_node.add_string(
                        //
                        ranking_custom,
                        self.suffix_len + prefix.len(),
                        true,
                        s_bytes,
                        is_custom_vec,
                        verbose,
                    );
                }

                let prefix_first_letter = prefix[0];
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
                    ls_index,
                    ls_size,
                    is_custom_ls,
                    s_bytes,
                    is_custom_vec,
                    verbose,
                );
            }
            PrefixTrieData::Vec(_) => {
                // This type "PrefixTrieData::Vec" is only used from the Shrink Phase and after.
                // Should never happen...
                // exit(0x0100);
            }
        }
    }
    fn update_rankings(&mut self, ls_index: usize, is_custom_ls: bool, s_bytes: &[u8]) {
        if is_custom_ls {
            // TODO: Improve Binary Search
            let custom_gs = &s_bytes[ls_index..];
            let mut p = 0;
            let mut q = self.rankings.len();
            while p < q {
                let mid = (q + p) / 2;
                let mid_gs_index = self.rankings[mid];
                let mid_gs = &s_bytes[mid_gs_index..];
                // TODO: Monitor string compare
                if custom_gs < mid_gs {
                    q = mid;
                } else {
                    p = mid + 1;
                }
            }
            if p == q {
                self.rankings.insert(p, ls_index);
            } else {
                // Should never happen...
                // exit(0x0100);
            }
        } else {
            self.rankings.push(ls_index);
        }
    }

    // SHRINK PHASE
    fn is_bridge_node(&self) -> bool {
        // Make sure to perform "shrink" before the "Merge Rankings and Sort" phase
        self.rankings.is_empty()
    }
    pub fn shrink(&mut self, prog_sa: &mut ProgSuffixArray) -> usize {
        // Note: After "shrink" the only Bridge Node will be the Root Node :)
        let mut next_id = 0;
        self.shrink_(&mut next_id, prog_sa);

        // Returning the Nodes Count
        next_id
    }
    fn shrink_(&mut self, next_id: &mut usize, prog_sa: &mut ProgSuffixArray) {
        // Node "self" ID (following pre-order traversal, so like DFS visits)
        if self.suffix_len == 0 || !self.is_bridge_node() {
            self.id = *next_id;
            *next_id += 1;

            prog_sa.assign_rankings_to_node_index(self.id, &mut self.rankings);
        }

        // Shrink Children's Children if they are Bridges
        match &mut self.data {
            PrefixTrieData::Leaf => {}
            PrefixTrieData::Children(children) => {
                let mut become_vec = false;
                for (_, child_node) in &mut *children {
                    if child_node.is_bridge_node() {
                        become_vec = true;
                    }
                    child_node.shrink_(next_id, prog_sa);
                }
                // Shrink Children if they are Bridges
                if become_vec {
                    let mut children_list_char_key = Vec::new();
                    for (&char_key, _) in &mut *children {
                        children_list_char_key.push(char_key);
                    }
                    let mut children_list_child_node = Vec::new();
                    for char_key in children_list_char_key {
                        let child_node = children.remove(&char_key).unwrap();
                        children_list_child_node.push(child_node);
                    }
                    let mut vec = Vec::new();
                    for child_node in children_list_child_node {
                        if child_node.is_bridge_node() {
                            // This is a Bridge Node, so consider directly its Children.
                            match child_node.data {
                                PrefixTrieData::Leaf => {
                                    // Should never happen...
                                    // exit(0x0100);
                                }
                                PrefixTrieData::Children(children) => {
                                    let children_list = children.into_values();
                                    vec.extend(children_list);
                                }
                                PrefixTrieData::DirectChild((_, child_node)) => {
                                    vec.push(*child_node);
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
            PrefixTrieData::DirectChild((_, child_node)) => {
                child_node.shrink_(next_id, prog_sa);
            }
            PrefixTrieData::Vec(_) => {
                // Should never happen...
                // exit(0x0100);
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
