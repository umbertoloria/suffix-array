use crate::suffix_array::chunking::get_max_size;
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::collections::BTreeMap;

pub fn create_prefix_trie(
    src: &str,
    src_length: usize,
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
) -> PrefixTrie {
    let custom_max_size =
        get_max_size(&custom_indexes, src_length).expect("custom_max_size is not valid");

    let mut root = PrefixTrie {
        label: "\0".into(),
        sons: BTreeMap::new(),
        rankings_canonical: Vec::new(),
        rankings_custom: Vec::new(),
        // rankings: Vec::new(),
        wbsa_p: 0,
        wbsa_q: 0,
        suffix_len: 0,
        min_father: None,
        max_father: None,
    };

    let custom_indexes_len = custom_indexes.len();

    for curr_suffix_length in 1..custom_max_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "curr_suffix_length" and,
        // if there exist, takes their Local Suffixes of "curr_suffix_length" length.
        let mut ordered_list_of_custom_factor_local_suffix_index = Vec::new();

        // Last Custom Factor
        let curr_custom_factor_len = src_length - custom_indexes[custom_indexes_len - 1];
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
        for custom_factor_local_suffix_index in ordered_list_of_custom_factor_local_suffix_index {
            // Implementation of "add_in_custom_prefix_trie".
            let local_suffix = &src[custom_factor_local_suffix_index
                ..custom_factor_local_suffix_index + curr_suffix_length];
            let chars_local_suffix = local_suffix.chars().collect::<Vec<_>>();

            let mut curr_node = &mut root;

            let mut i_chars_of_suffix = 0; // This is the current "depth" of "curr_node".
            while i_chars_of_suffix < curr_suffix_length {
                let curr_letter = chars_local_suffix[i_chars_of_suffix];

                if !curr_node.sons.contains_key(&curr_letter) {
                    // First time "curr_node" node deals with "curr_letter".
                    curr_node.sons.insert(
                        curr_letter,
                        PrefixTrie {
                            label: format!("{}{}", curr_node.label, curr_letter),
                            sons: BTreeMap::new(),
                            rankings_canonical: Vec::new(),
                            rankings_custom: Vec::new(),
                            // rankings: Vec::new(),
                            wbsa_p: 0,
                            wbsa_q: 0,
                            suffix_len: i_chars_of_suffix + 1,
                            min_father: None,
                            max_father: None,
                        },
                    );
                }
                curr_node = curr_node.sons.get_mut(&curr_letter).unwrap();

                i_chars_of_suffix += 1;
            }
            // TODO: Here we could create an interesting wrapping among real "non-bridge" nodes
            if is_custom_vec[custom_factor_local_suffix_index] {
                curr_node
                    .rankings_custom
                    .push(custom_factor_local_suffix_index);
            } else {
                curr_node
                    .rankings_canonical
                    .push(custom_factor_local_suffix_index);
            }
        }
    }

    root
}

pub struct PrefixTrie {
    pub label: String,
    // TODO: Try to use HashMap but keeping chars sorted
    pub sons: BTreeMap<char, PrefixTrie>,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    // pub rankings: Vec<usize>,
    pub wbsa_p: usize, // Incl.
    pub wbsa_q: usize, // Excl.
    pub suffix_len: usize,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTrie {
    pub fn merge_rankings_and_sort_recursive(
        &mut self,
        src: &str,
        wbsa: &mut Vec<usize>,
        wbsa_start_from_index: usize,
    ) -> usize {
        // Single "rankings" list
        let mut new_rankings = Vec::new();
        for &local_suffix_index in &self.rankings_canonical {
            new_rankings.push((local_suffix_index, &src[local_suffix_index..]));
        }
        for &local_suffix_index in &self.rankings_custom {
            new_rankings.push((local_suffix_index, &src[local_suffix_index..]));
        }

        let mut p = wbsa_start_from_index;
        self.wbsa_p = p;
        if !new_rankings.is_empty() {
            // TODO: Maybe sorting is sometimes avoidable
            sort_pair_vector_of_indexed_strings(&mut new_rankings);
            // Update list only if strings were actually sorted and moved.
            /*self.rankings.clear();
            for (index, _) in new_rankings {
                self.rankings.push(index);
            }
            for &ranking in &self.rankings {*/
            for (index, _) in new_rankings {
                wbsa[p] = index;
                p += 1;
            }
        }
        self.wbsa_q = p;

        // Recursive calls...
        for (_, son) in &mut self.sons {
            let new_p = son.merge_rankings_and_sort_recursive(src, wbsa, p);
            p = new_p;
        }

        p
    }
    /*pub fn in_prefix_merge_upon_father_node(
        &mut self,
        src: &str,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) {
        let father_rankings = &self.rankings;
        for (_, child) in &mut self.sons {
            // println!("IN-PREFIX MERGE: update Child Rankings List");
            // println!(" > Father: \"{}\" {:?}", self.label, self.rankings);
            // println!(" > Child : \"{}\" {:?}", child.label, child.rankings);

            // Managing "bridge" nodes
            let mut child_rankings = &mut child.rankings;
            if child_rankings.is_empty() {
                // TODO: Solve bug of bridge nodes
                continue;
                /*
                let mut list_of_rankings = Vec::new();
                for (_, nephew) in &child.sons {
                    let mut cloned_rankings = nephew.rankings.clone();
                    // Variable "cloned_rankings" will be wiped after the "append".
                    list_of_rankings.append(&mut cloned_rankings);
                }
                &child_rankings.append(&mut list_of_rankings);
                if child_rankings.is_empty() {
                    // Should never happen...
                    exit(0x0100);
                }
                */
            }
            let child_rankings = child_rankings;

            // First Child Suffix create (for the next comparisons...)
            let curr_suffix_size = child.suffix_len;
            // TODO: Safe slice like with "cmp2_from_father"?
            let cmp1_from_child = &src[child_rankings[0]..child_rankings[0] + curr_suffix_size];
            // println!(" > first child suffix=\"{cmp1_from_child}\"");

            // Calculating MIN-FATHER
            // This is an index from father rankings that tells from what element the following
            // statement holds:
            // Child First Suffix (len child) <= Father Suffix (len child)
            let mut min_father = None;
            // TODO: If this is not the First Child, we could use the previous min/max father values
            //  instead of starting from the very left of Father Rankings again
            // println!(" > calculating MIN-FATHER");
            for i in 0..father_rankings.len() {
                let cmp2_from_father = &src[father_rankings[i]
                    ..usize::min(father_rankings[i] + curr_suffix_size, src.len())];
                // println!(
                //     "   > curr [i={}, ranking={}] father suffix=\"{}\", is it >= our first child suffix?",
                //     i, father_rankings[i], cmp2_from_father
                // );
                if cmp1_from_child <= cmp2_from_father {
                    // println!("       > yes it is => updated min_father={i}");
                    min_father = Some(i);
                    break;
                }
            }
            // println!("   > MIN-FATHER={:?}", min_father);
            if min_father.is_none() {
                // println!();
                continue;
            }
            let min_father = min_father.unwrap();
            child.min_father = Some(min_father);

            // Calculating MAX-FATHER
            // This is an index from father rankings (starting from MIN-FATHER) that tells from what
            // element the following statement holds:
            // Child First Suffix (len child) < Father Suffix (len child)
            // This means that the elements from MIN-FATHER incl. to MAX-FATHER excl. should be
            // compared two-by-two with all Child Suffixes, in order to update Child Suffixes List.
            // println!(" > calculating MAX-FATHER");
            // println!("   > first child suffix=\"{cmp1_from_child}\"");
            // println!(" > first compare to see if MAX-FATHER should be -1 or if there are items =");
            // println!(
            //     "   > father_ranking[{}]: {}",
            //     min_father, father_rankings[min_father]
            // );
            let cmp2_from_father =
                &src[father_rankings[min_father]..father_rankings[min_father] + curr_suffix_size];
            // println!(
            //     "   > curr [i=MIN-FATHER={}] father suffix=\"{}\", is it > our first child suffix?",
            //     min_father, cmp2_from_father
            // );
            if cmp1_from_child < cmp2_from_father {
                // This means there are no Father Suffixes starting from MIN-FATHER incl. that are
                // equal to the First Child Suffix, to MIN-FATHER=-1.
                // println!("       > yes it is => setting MAX-FATHER=None()\n");
                continue;
            }
            // Ok, if we are here it means that there is at least one Father Suffix that is equals
            // to the First Child Suffix. So we're looking for the first (if there is) Father Suffix
            // that is greater that the First Child Suffix.
            let mut i = min_father;
            while i < father_rankings.len() {
                let cmp2_from_father =
                    &src[father_rankings[i]..father_rankings[i] + curr_suffix_size];
                // println!(
                //     "   > curr [i={}] father suffix=\"{}\", is it > our first child suffix?",
                //     i, cmp2_from_father
                // );
                if cmp1_from_child != cmp2_from_father {
                    // println!("       > yes it is => found what we needed");
                    break;
                }
                i += 1;
            }
            let max_father = i;
            // println!("       > setting MAX-FATHER={:?}", max_father);
            child.max_father = Some(max_father);

            // Update CHILD RANKINGS
            // Here we compare two-by-two the Suffix Window between:
            // * MIN-FATHER incl. to MAX-FATHER excl., and
            // * the entire Child Suffixes List.
            let mut overwrite_child_rankings = Vec::new();
            // TODO: Preallocate to "(max_father-min_father)+1+child.size()" (legacy code...)
            i = min_father;
            let mut j = 0;
            while i < max_father && j < child_rankings.len() {
                let x = father_rankings[i];
                let y = child_rankings[j];
                let result_rules = Self::rules(
                    x,
                    y,
                    curr_suffix_size,
                    src,
                    &icfl_indexes,
                    &is_custom_vec,
                    &factor_list,
                );
                if !result_rules {
                    overwrite_child_rankings.push(father_rankings[i]);
                    i += 1;
                } else {
                    overwrite_child_rankings.push(child_rankings[j]);
                    j += 1;
                }
            }
            // Then we put in list the remaining Child Suffixes...
            while j < child_rankings.len() {
                overwrite_child_rankings.push(child_rankings[j]);
                j += 1;
            }
            // ...and then we put in list the remaining Father Suffixes until MAX-FATHER (excl.).
            while i < max_father {
                overwrite_child_rankings.push(father_rankings[i]);
                i += 1;
            }

            child_rankings.clear();
            child_rankings.append(&mut overwrite_child_rankings);
            // println!("Updated Child Rankings List = {:?}", child_rankings);
        }
    }
    pub fn in_prefix_merge_bit_vector(
        &mut self,
        src: &str,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) {
        self.in_prefix_merge_upon_father_node(src, icfl_indexes, is_custom_vec, factor_list);
        // Recursive calls...
        for (_, child) in &mut self.sons {
            child.in_prefix_merge_bit_vector(src, icfl_indexes, is_custom_vec, factor_list);
        }
    }*/
    pub fn shrink_bottom_up(&mut self) {
        if self.sons.is_empty() {
            println!("SHRINK THE LEAF \"{}\"", self.label);
        } else {
            for (_, son) in &mut self.sons {
                son.shrink_bottom_up();
            }
            if self.wbsa_p < self.wbsa_q {
                println!("SHRINK MERGING SONS OF \"{}\"", self.label,);
            }
        }
    }
    fn rules(
        x: usize,
        y: usize,
        child_offset: usize,
        src: &str,
        icfl_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) -> bool {
        let icfl_list_size = icfl_list.len();
        if is_custom_vec[x] && is_custom_vec[y] {
            let cmp1 = &src[y + child_offset..];
            let cmp2 = &src[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }
        } else if is_custom_vec[x] {
            if factor_list[x] <= factor_list[y] {
                if y >= icfl_list[icfl_list_size - 1] {
                    true
                } else {
                    false
                }
            } else {
                let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }
            }
        } else if is_custom_vec[y] {
            if factor_list[y] <= factor_list[x] {
                if x >= icfl_list[icfl_list_size - 1] {
                    false
                } else {
                    true
                }
            } else {
                let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }
            }
        } else if x >= icfl_list[icfl_list_size - 1] && y >= icfl_list[icfl_list_size - 1] {
            false
        } else if factor_list[x] == factor_list[y] {
            true
        } else {
            if x >= icfl_list[icfl_list_size - 1] {
                false
            } else if y >= icfl_list[icfl_list_size - 1] {
                let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }
            } else {
                if x > y {
                    true
                } else {
                    let cmp1 = &src[y + child_offset..];
                    let cmp2 = &src[x + child_offset..];
                    if cmp1 < cmp2 {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
    pub fn print(&self, tabs_offset: usize, prefix: String) {
        /*if self.sons.len() == 1 {
            let char_key = self.sons.keys().collect::<Vec<_>>()[0];
            self.sons
                .get(char_key)
                .unwrap()
                .print(tabs_offset, format!("{}{}", prefix, char_key));
        } else {*/
        println!(
            "{}|{:2}: \"{}\" {}, min= {}, MAX= {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            prefix,
            // self.label,
            format!("{:?} {:?}", self.rankings_canonical, self.rankings_custom),
            if let Some(min_father) = self.min_father {
                min_father as i16
            } else {
                -1
            },
            if let Some(max_father) = self.max_father {
                max_father as i16
            } else {
                -1
            },
        );
        for (char_key, node) in &self.sons {
            node.print(tabs_offset + 1, format!("{}{}", prefix, char_key));
        }
        // }
    }
    pub fn print_with_wbsa(&self, tabs_offset: usize, prefix: String, wbsa: &Vec<usize>) {
        /*if self.sons.len() == 1 {
            let char_key = self.sons.keys().collect::<Vec<_>>()[0];
            self.sons
                .get(char_key)
                .unwrap()
                .print(tabs_offset, format!("{}{}", prefix, char_key));
        } else {*/
        let rankings = &wbsa[self.wbsa_p..self.wbsa_q];
        println!(
            "{}\"{}\" {}   [{}..{})",
            "\t".repeat(tabs_offset),
            prefix,
            // self.label,
            format!("{:?}", rankings),
            self.wbsa_p,
            self.wbsa_q,
        );
        for (char_key, node) in &self.sons {
            node.print_with_wbsa(tabs_offset + 1, format!("{}{}", prefix, char_key), wbsa);
        }
        // }
    }
}
