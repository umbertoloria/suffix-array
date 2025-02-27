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
        suffix_len: 0,
        sons: BTreeMap::new(),
        rankings_canonical: Vec::new(),
        rankings_custom: Vec::new(),
        // rankings: Vec::new(),
        wbsa_p: 0,
        wbsa_q: 0,
        shrunk: false,
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
                            suffix_len: i_chars_of_suffix + 1,
                            sons: BTreeMap::new(),
                            rankings_canonical: Vec::new(),
                            rankings_custom: Vec::new(),
                            // rankings: Vec::new(),
                            wbsa_p: 0,
                            wbsa_q: 0,
                            shrunk: false,
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
    pub suffix_len: usize,
    // TODO: Try to use HashMap but keeping chars sorted
    pub sons: BTreeMap<char, PrefixTrie>,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    // pub rankings: Vec<usize>,
    pub wbsa_p: usize, // Incl.
    pub wbsa_q: usize, // Excl.
    pub shrunk: bool,
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
    pub fn shrink_bottom_up(
        &mut self,
        wbsa: &mut Vec<usize>,
        src: &str,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) {
        if self.shrunk {
            return;
        }
        if self.sons.is_empty() {
            self.shrunk = true;
            /*println!(
                "SHRINK THE LEAF \"{}\" => {:?} (from {} to {})",
                self.label, &wbsa[self.wbsa_p..self.wbsa_q], self.wbsa_p, self.wbsa_q
            );*/
        } else {
            // Shrink sons
            for (_, son) in &mut self.sons {
                son.shrink_bottom_up(wbsa, src, icfl_indexes, is_custom_vec, factor_list);
            }
            let max_wbsa_q = self.get_max_wbsa_q();
            /*println!(
                "SHRINK MERGING SONS OF \"{}\": from {} to {} (extended to {})",
                self.label, self.wbsa_p, self.wbsa_q, max_wbsa_q
            );*/

            if self.suffix_len > 0 {
                // Merge and Sort current Rankings

                if self.wbsa_p == self.wbsa_q {
                    // This is a "bridge" node, to we take the Rankings or the son in order.

                    // TODO: Demonstrate this
                    self.wbsa_q = max_wbsa_q;
                    self.shrunk = true;

                    self.sons.clear();

                    // println!(" > \"bridge\" node {} fused with sons", self.label);
                } else {
                    let sons = &self.sons.values().collect::<Vec<_>>();

                    // Pre-dimensioning the auxiliary memory for new Node's Rankings calculation
                    let mut children_rankings_count = 0;
                    for son in sons {
                        children_rankings_count += son.wbsa_q - son.wbsa_p;
                    }
                    let mut result = Vec::with_capacity(
                        // Father Rankings count
                        (self.wbsa_q - self.wbsa_p)
                        // Children Rankings count
                        + children_rankings_count,
                    );

                    let mut i_father_index = self.wbsa_p;
                    for son in sons {
                        // Start by comparing Father Suffixes (using the length of this son, if
                        // possible) and putting first the ones that are < Child Suffix.
                        let child_suffix_len = son.suffix_len;
                        let child_ls = &src[wbsa[son.wbsa_p]..wbsa[son.wbsa_p] + child_suffix_len];
                        /*println!(
                            " > merge father={} {:?} with child={} {:?}",
                            self.label,
                            &wbsa[self.wbsa_p..self.wbsa_q],
                            son.label,
                            &wbsa[son.wbsa_p..son.wbsa_q],
                        );*/

                        while i_father_index < self.wbsa_q {
                            let curr_father_ls_index = wbsa[i_father_index];
                            let curr_father_ls = &src[curr_father_ls_index
                                ..usize::min(curr_father_ls_index + child_suffix_len, src.len())];

                            // Comparing strings.
                            if curr_father_ls < child_ls {
                                result.push(curr_father_ls_index);
                                i_father_index += 1;
                            } else {
                                // Found a Father Suffix that is >= Child Suffix.
                                break;
                            }
                        }

                        // From now, for all Father Suffixes that we'll encounter will always hold:
                        //  -> Curr. Father Suffix >= Curr. Child Suffix
                        // We'll use "RULES" to manage comparisons between pairs that are "equal".
                        // This means that the real differences are only beyond these suffixes, so
                        // considering them as Global Suffixes and not Local Suffixes.

                        // These comparisons using "RULES" are only valid until:
                        //  -> Curr. Father Suffix == Curr. Child Suffix
                        // Let's find out the max index, after which we no longer have
                        // Curr. Father Suffixes that are equal to Curr. Child Suffix, and lose the
                        // possibility to use "RULES".

                        let mut max_i_father_index = i_father_index;
                        while max_i_father_index < self.wbsa_q {
                            let curr_father_ls_index = wbsa[max_i_father_index];
                            let curr_father_ls = &src[curr_father_ls_index
                                ..usize::min(curr_father_ls_index + child_suffix_len, src.len())];

                            // Comparing strings.
                            if curr_father_ls > child_ls {
                                // Found Father Suffix that is > Curr. Child Suffix.
                                break;
                            } else {
                                max_i_father_index += 1;
                            }
                        }

                        // Ok, now we can use "RULES" for all items between "i_father_index" (incl.)
                        // and "max_i_father_index" (excl.).
                        let mut j_child_index = son.wbsa_p;
                        while i_father_index < max_i_father_index && j_child_index < son.wbsa_q {
                            let curr_father_ls_index = wbsa[i_father_index];
                            let curr_child_ls_index = wbsa[j_child_index];
                            let result_rules = Self::rules(
                                curr_father_ls_index,
                                curr_child_ls_index,
                                child_suffix_len,
                                src,
                                &icfl_indexes,
                                &is_custom_vec,
                                &factor_list,
                            );
                            if !result_rules {
                                result.push(curr_father_ls_index);
                                i_father_index += 1;
                            } else {
                                result.push(curr_child_ls_index);
                                j_child_index += 1;
                            }
                        }
                        // Ok, we first take all Child Suffixes left, then continue to insert all
                        // Father Suffixes left.
                        while j_child_index < son.wbsa_q {
                            result.push(wbsa[j_child_index]);
                            j_child_index += 1;
                        }
                    }
                    // Ok, we now insert all Father Suffixes left.
                    while i_father_index < self.wbsa_q {
                        result.push(wbsa[i_father_index]);
                        i_father_index += 1;
                    }

                    // Here we finally apply "result" data into the "Wanna Be Suffix Array" :)
                    self.wbsa_q = self.wbsa_p + result.len();
                    i_father_index = self.wbsa_p;
                    for result_item in result {
                        wbsa[i_father_index] = result_item;
                        i_father_index += 1;
                    }
                    self.shrunk = true;

                    self.sons.clear();
                    /*println!(
                        "   > done with result={:?}",
                        &wbsa[self.wbsa_p..self.wbsa_q]
                    );*/
                }
            } else {
                // This is the Root Node. Useless to merge First Level Children since the
                // "Wanna Be Suffix Array" is already fully computed :)
            }
        }
    }
    fn get_max_wbsa_q(&self) -> usize {
        if self.sons.is_empty() {
            self.wbsa_q
        } else {
            let sons = &self.sons.values().collect::<Vec<_>>();
            let last_son = sons[sons.len() - 1];
            last_son.get_max_wbsa_q()
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
