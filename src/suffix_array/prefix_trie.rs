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
        wbsa_p: 0,
        wbsa_q: 0,
        shrunk: false,
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
                            wbsa_p: 0,
                            wbsa_q: 0,
                            shrunk: false,
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
    pub wbsa_p: usize, // Incl.
    pub wbsa_q: usize, // Excl.
    pub shrunk: bool,
}
impl PrefixTrie {
    // Getters
    fn get_buff_index_left(&self) -> usize {
        self.wbsa_p
    }
    fn get_buff_index_right_excl(&self) -> usize {
        self.wbsa_q
    }
    fn get_rankings_count(&self) -> usize {
        self.get_buff_index_right_excl() - self.get_buff_index_left()
    }
    fn get_max_buff_index_right_excl_from_righted_child(&self) -> usize {
        if self.sons.is_empty() {
            self.get_buff_index_right_excl()
        } else {
            let sons = &self.sons.values().collect::<Vec<_>>();
            let last_son = sons[sons.len() - 1];
            last_son.get_max_buff_index_right_excl_from_righted_child()
        }
    }
    fn get_first_ls_index(&self, wbsa: &Vec<usize>) -> usize {
        wbsa[self.get_buff_index_left()]
    }
    fn get_rankings<'a>(&self, wbsa: &'a Vec<usize>) -> &'a [usize] {
        &wbsa[self.get_buff_index_left()..self.get_buff_index_right_excl()]
    }
    fn get_string_from_first_ranking_with_length<'a>(
        &self,
        wbsa: &Vec<usize>,
        str: &'a str,
        string_length: usize,
    ) -> &'a str {
        let child_ls_index = self.get_first_ls_index(wbsa);
        &str[child_ls_index..child_ls_index + string_length]
    }

    // Prints
    pub fn print(&self, tabs_offset: usize, prefix: String) {
        /*if self.sons.len() == 1 {
            let char_key = self.sons.keys().collect::<Vec<_>>()[0];
            self.sons
                .get(char_key)
                .unwrap()
                .print(tabs_offset, format!("{}{}", prefix, char_key));
        } else {*/
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            prefix,
            // self.label,
            format!("{:?} {:?}", self.rankings_canonical, self.rankings_custom),
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
        println!(
            "{}\"{}\" {:?}   [{}..{})",
            "\t".repeat(tabs_offset),
            prefix,
            // self.label,
            self.get_rankings(wbsa),
            self.get_buff_index_left(),
            self.get_buff_index_right_excl(),
        );
        for (char_key, node) in &self.sons {
            node.print_with_wbsa(tabs_offset + 1, format!("{}{}", prefix, char_key), wbsa);
        }
        // }
    }

    // Tree transformation
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
            // Was already shrunk...
            return;
        }
        if self.sons.is_empty() {
            // Shrinking is easy for a Left Node since there's no merging to do with Children.
            self.shrunk = true;
            /*println!(
                "SHRINK THE LEAF \"{}\" => {:?} (from {} to {})",
                self.label,
                self.get_rankings(wbsa),
                self.get_buff_index_left(),
                self.get_buff_index_right_excl()
            );*/
            return;
        }

        // Shrink sons
        for (_, son) in &mut self.sons {
            son.shrink_bottom_up(wbsa, src, icfl_indexes, is_custom_vec, factor_list);
        }
        /*println!(
            "SHRINK MERGING SONS OF \"{}\": from {} to {} (extended to {})",
            self.label,
            self.get_buff_index_left(),
            self.get_buff_index_right_excl(),
            self.get_max_last_excl_wbsa_index_from_last_child()
        );*/

        if self.suffix_len == 0 {
            // This is the Root Node. Useless to merge First Level Children since the
            // "Wanna Be Suffix Array" is already fully computed :)
            return;
        }

        // Merge and Sort current Rankings
        if self.get_rankings_count() == 0 {
            // This is a "bridge" node, to we take the Rankings or the son in order.

            // TODO: Demonstrate this
            self.wbsa_q = self.get_max_buff_index_right_excl_from_righted_child();
            self.shrunk = true;

            self.sons.clear();

            // println!(" > \"bridge\" node {} fused with sons", self.label);
        } else {
            let sons = &self.sons.values().collect::<Vec<_>>();

            // Pre-dimensioning the auxiliary memory for new Node's Rankings calculation
            let mut children_rankings_count = 0;
            for son in sons {
                children_rankings_count += son.get_rankings_count();
            }
            let mut result =
                Vec::with_capacity(self.get_rankings_count() + children_rankings_count);

            let mut i_father_index = self.get_buff_index_left();
            for son in sons {
                // Start by comparing Father Suffixes (using the length of this son, if
                // possible) and putting first the ones that are < Child Suffix.
                let child_suffix_len = son.suffix_len;
                let child_ls =
                    son.get_string_from_first_ranking_with_length(wbsa, src, child_suffix_len);
                /*let child_ls = &src
                [wbsa[son.wbsa_p]..usize::min(wbsa[son.wbsa_p] + child_suffix_len, src.len())];*/
                /*println!(
                    " > merge father={} {:?} with child={} {:?}",
                    self.label,
                    self.get_rankings(wbsa),
                    son.label,
                    son.get_rankings(wbsa),
                );*/

                // println!("   > phase 1: first father's smaller than child");
                while i_father_index < self.get_buff_index_right_excl() {
                    let curr_father_ls_index = wbsa[i_father_index];
                    let curr_father_ls = &src[curr_father_ls_index
                        ..usize::min(curr_father_ls_index + child_suffix_len, src.len())];

                    // Comparing strings.
                    if curr_father_ls < child_ls {
                        result.push(curr_father_ls_index);
                        // println!("   > father ls index {} added first", curr_father_ls_index);
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

                // println!("   > phase 2: window for comparing using \"RULES\"");
                let mut max_i_father_index = i_father_index;
                while max_i_father_index < self.get_buff_index_right_excl() {
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
                // println!("     > [{}, {})", i_father_index, max_i_father_index);

                // Ok, now we can use "RULES" for all items between "i_father_index" (incl.)
                // and "max_i_father_index" (excl.).
                let mut j_child_index = son.get_buff_index_left();
                while i_father_index < max_i_father_index
                    && j_child_index < son.get_buff_index_right_excl()
                {
                    let curr_father_ls_index = wbsa[i_father_index];
                    let curr_child_ls_index = wbsa[j_child_index];
                    // FIXME: The value "child_suffix_len" should be the same as what were
                    //  saved in its Native Node. Shrinking should preserve that
                    //  Child Suffix Length, otherwise there's a bug :(
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
                        /*println!(
                            "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                            &src
                                [curr_father_ls_index..curr_father_ls_index + child_suffix_len], curr_father_ls_index, &src
                                    [curr_child_ls_index..curr_child_ls_index + child_suffix_len], curr_child_ls_index, child_suffix_len
                        );*/
                        result.push(curr_father_ls_index);
                        i_father_index += 1;
                    } else {
                        /*println!(
                            "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: son wins",
                            &src
                                [curr_father_ls_index..curr_father_ls_index + child_suffix_len], curr_father_ls_index, &src
                                [curr_child_ls_index..curr_child_ls_index + child_suffix_len], curr_child_ls_index, child_suffix_len
                        );*/
                        result.push(curr_child_ls_index);
                        j_child_index += 1;
                    }
                }
                // Ok, we first take all Child Suffixes left, then continue to insert all
                // Father Suffixes left.
                // println!("   > phase 3: then the last father's");
                while j_child_index < son.get_buff_index_right_excl() {
                    result.push(wbsa[j_child_index]);
                    j_child_index += 1;
                }
            }
            // Ok, we now insert all Father Suffixes left.
            while i_father_index < self.get_buff_index_right_excl() {
                result.push(wbsa[i_father_index]);
                i_father_index += 1;
            }

            // Here we finally apply "result" data into the "Wanna Be Suffix Array" :)
            self.wbsa_q = self.wbsa_p + result.len();
            i_father_index = self.get_buff_index_left();
            for result_item in result {
                wbsa[i_father_index] = result_item;
                i_father_index += 1;
            }
            self.shrunk = true;

            self.sons.clear();
            // println!("   > done with result={:?}", self.get_rankings(wbsa));
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
}
