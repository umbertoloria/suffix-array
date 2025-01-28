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
        min_father: None,
        max_father: None,
        rankings_forced: None,
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
                            min_father: None,
                            max_father: None,
                            rankings_forced: None,
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
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
    pub rankings_forced: Option<Vec<usize>>,
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
            "{}\"{}\" {:?}   min={}, MAX={}",
            "\t".repeat(tabs_offset),
            prefix,
            // self.label,
            self.get_real_rankings(wbsa),
            // self.get_buff_index_left(),
            // self.get_buff_index_right_excl(),
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
            node.print_with_wbsa(tabs_offset + 1, format!("{}{}", prefix, char_key), wbsa);
        }
        // }
    }

    // Tree transformation
    pub fn merge_rankings_and_sort_recursive(
        &mut self,
        src: &str,
        wbsa: &mut Vec<usize>,
        depths: &mut Vec<usize>,
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

        // Depth
        for i in self.get_buff_index_left()..self.get_buff_index_right_excl() {
            let ls_index = wbsa[i];
            depths[ls_index] = self.suffix_len;
        }

        // Recursive calls...
        for (_, son) in &mut self.sons {
            let new_p = son.merge_rankings_and_sort_recursive(src, wbsa, depths, p);
            p = new_p;
        }

        p
    }
    pub fn in_prefix_merge(
        &mut self,
        str: &str,
        wbsa: &mut Vec<usize>,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
        verbose: bool,
    ) {
        if self.suffix_len == 0 {
            // This is the Root Node.
            for son in self.sons.values_mut() {
                son.in_prefix_merge(
                    str,
                    wbsa,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    factor_list,
                    verbose,
                );
            }
            return;
        }

        if self.get_rankings_count() == 0 {
            // This is a Bridge Node.
            for son in self.sons.values_mut() {
                son.in_prefix_merge(
                    str,
                    wbsa,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    factor_list,
                    verbose,
                );
            }
            return;
        }

        // Node with Rankings.
        let this_left = self.get_buff_index_left();
        let this_right_excl = self.get_buff_index_right_excl();
        for son in self.sons.values_mut() {
            son.in_prefix_merge_deep(
                str,
                wbsa,
                depths,
                icfl_indexes,
                is_custom_vec,
                factor_list,
                this_left,
                this_right_excl,
                verbose,
            );
        }
    }
    fn in_prefix_merge_deep(
        &mut self,
        str: &str,
        wbsa: &mut Vec<usize>,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
        parent_buff_index_left: usize,
        parent_buff_index_right_excl: usize,
        verbose: bool,
    ) {
        // Parent has to *ALWAYS* have rankings.

        if self.get_rankings_count() == 0 {
            // This is a Bridge Node.
            for son in self.sons.values_mut() {
                son.in_prefix_merge_deep(
                    str,
                    wbsa,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    factor_list,
                    parent_buff_index_left,
                    parent_buff_index_right_excl,
                    verbose,
                );
            }
            return;
        }

        // Compare this node's rankings with parent's rankings.
        let parent_rankings = &wbsa[parent_buff_index_left..parent_buff_index_right_excl];
        let parent_first_ls_index = parent_rankings[0];
        let parent_ls_length = depths[parent_first_ls_index];
        let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];

        let this_rankings = self.get_rankings(wbsa);
        let this_first_ls_index = this_rankings[0];
        let this_ls_length = depths[this_first_ls_index];
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
            println!(
                "Compare parent ({}) {:?} with curr ({}) {:?}",
                parent_ls, parent_rankings, this_ls, this_rankings
            );
        }

        // MERGE RANKINGS
        let mut i_parent = 0;

        while i_parent < parent_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            if curr_parent_ls < this_ls {
                // Go ahead, this part of Parent Rankings has LSs that are < than Curr LS.
                i_parent += 1;
            } else {
                // Found a Parent LS that is >= Curr LS.
                self.min_father = Some(i_parent);
                break;
            }
        }
        if i_parent >= parent_rankings.len() {
            // This means "min_father"=None and "max_father"=None.
        } else {
            // From here, we have a "min_father" value.

            // let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            if curr_parent_ls > this_ls {
                // This means "max_father"=None.
                // There's no Window for Comparing Rankings using "RULES".
            } else {
                while i_parent < parent_rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_parent_ls = &str[curr_parent_ls_index
                        ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
                    if curr_parent_ls == this_ls {
                        // Go ahead, this part of Parent Rankings has LSs that are = than Curr LS.
                        self.max_father = Some(i_parent + 1);
                        i_parent += 1;
                    } else {
                        // Found a Parent LS that is > Curr LS.
                        break;
                    }
                }

                i_parent = self.min_father.unwrap();
                let mut j_this = 0;

                let mut result = Vec::new();
                if let Some(max_father) = self.max_father {
                    if verbose {
                        println!("   > start comparing, window=[{},{})", i_parent, max_father);
                    }
                    while i_parent < max_father && j_this < this_rankings.len() {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        let curr_this_ls_index = this_rankings[j_this];
                        let child_offset = self.suffix_len;
                        let result_rules = Self::rules_safe(
                            curr_parent_ls_index,
                            curr_this_ls_index,
                            child_offset,
                            str,
                            &icfl_indexes,
                            &is_custom_vec,
                            &factor_list,
                        );
                        if !result_rules {
                            if verbose {
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                                    &str
                                        [curr_parent_ls_index..curr_parent_ls_index + child_offset], curr_parent_ls_index, &str
                                        [curr_this_ls_index..curr_this_ls_index + child_offset], curr_this_ls_index, child_offset
                                );
                            }
                            result.push(curr_parent_ls_index);
                            i_parent += 1;
                        } else {
                            if verbose {
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: son wins",
                                    &str
                                        [curr_parent_ls_index..curr_parent_ls_index + child_offset], curr_parent_ls_index, &str
                                        [curr_this_ls_index..curr_this_ls_index + child_offset], curr_this_ls_index, child_offset
                                );
                            }
                            result.push(curr_this_ls_index);
                            j_this += 1;
                        }
                    }
                }
                while j_this < this_rankings.len() {
                    result.push(this_rankings[j_this]);
                    j_this += 1;
                }
                if let Some(max_father) = self.max_father {
                    while i_parent < max_father {
                        result.push(parent_rankings[i_parent]);
                        i_parent += 1;
                    }
                }
                self.rankings_forced = Some(result);
            }
        }

        // Now it's your turn to be the parent.
        let this_left = self.get_buff_index_left();
        let this_right_excl = self.get_buff_index_right_excl();
        for son in self.sons.values_mut() {
            son.in_prefix_merge_deep(
                str,
                wbsa,
                depths,
                icfl_indexes,
                is_custom_vec,
                factor_list,
                this_left,
                this_right_excl,
                verbose,
            );
        }
    }
    pub fn prepare_get_common_prefix_partition(
        &mut self,
        wbsa: &mut Vec<usize>,
        sa: &mut Vec<usize>,
    ) {
        // TODO: Make sure "self" is always "root".
        if self.sons.is_empty() {
            // TODO: This should never happen...
            return;
        }
        for first_layer_son in self.sons.values_mut() {
            let part_of_sa = first_layer_son.get_common_prefix_partition(wbsa);
            println!("{:?}", part_of_sa);
            sa.extend(part_of_sa);
        }
    }
    /*pub fn dump_onto_wbsa(&self, wbsa: &mut Vec<usize>, sa: &mut Vec<usize>, level: usize) {
        if self.suffix_len == 0 {
            // This is the Root Node.
            for son in self.sons.values() {
                son.dump_onto_wbsa(wbsa, sa, 0);
            }
            return;
        }

        if self.get_rankings_count() == 0 {
            // This is a Bridge Node.
            for son in self.sons.values() {
                son.dump_onto_wbsa(wbsa, sa, level + 1);
            }
            return;
        }

        // Node with Rankings.
        let this_real_rankings = self.get_real_rankings(wbsa);
        let mut i_parent = 0;
        for son in self.sons.values() {
            i_parent = son.dump_onto_wbsa_deep(wbsa, sa, this_real_rankings, i_parent, level + 1);
        }
        // Managing the rest of my rankings myself.
        while i_parent < this_real_rankings.len() {
            // +
            sa.push(this_real_rankings[i_parent]);
            print_with_offset(level, format!("**[0]** {}", this_real_rankings[i_parent]));
            // -

            i_parent += 1;
        }
    }
    fn dump_onto_wbsa_deep(
        &self,
        wbsa: &mut Vec<usize>,
        sa: &mut Vec<usize>,
        parent_real_rankings: &Vec<usize>,
        parent_index_start_from: usize,
        level: usize,
    ) -> usize {
        // Parent has to *ALWAYS* have rankings.

        let mut i_parent = parent_index_start_from;

        if self.get_rankings_count() == 0 {
            // This is a Bridge Node.
            print_with_offset(
                level,
                format!(
                    "dump: parent {:?} (from {}), curr is bridge",
                    &parent_real_rankings[parent_index_start_from..],
                    parent_index_start_from
                ),
            );
            for son in self.sons.values() {
                i_parent = son.dump_onto_wbsa_deep(wbsa, sa, parent_real_rankings, i_parent, level + 1);
            }

            return i_parent;
        }

        // Node with Rankings.
        let this_real_rankings = self.get_real_rankings(wbsa);
        print_with_offset(
            level,
            format!(
                "dump: parent {:?} (from {}), this {:?} with min={:?} and MAX={:?}",
                &parent_real_rankings[parent_index_start_from..],
                parent_index_start_from,
                this_real_rankings,
                self.min_father,
                self.max_father
            ),
        );
        if let Some(min_father) = self.min_father {
            while i_parent < min_father {
                // +
                sa.push(parent_real_rankings[i_parent]);
                print_with_offset(level, format!("**[A]** {}", parent_real_rankings[i_parent]));
                // -

                i_parent += 1;
            }
        } else {
            // If Min Father is None, then use first all Parent Rankings and then all Curr Rankings.
            while i_parent < parent_real_rankings.len() {
                // +
                sa.push(parent_real_rankings[i_parent]);
                print_with_offset(level, format!("**[D]** {}", parent_real_rankings[i_parent]));
                // -

                i_parent += 1;
            }
            let mut j_curr = 0;
            for son in self.sons.values() {
                print_with_offset(level, format!("> valuta figlio"));
                j_curr = son.dump_onto_wbsa_deep(wbsa, sa, this_real_rankings, j_curr, level + 1);
            }
        }

        if self.sons.is_empty() {
            // Managing my rankings myself.
            for i in 0..this_real_rankings.len() {
                // +
                sa.push(this_real_rankings[i]);
                print_with_offset(level, format!("**[B]** {}", this_real_rankings[i]));
                // -
            }

            if let Some(max_father) = self.max_father {
                max_father
            } else {
                // Min Father NONE, means we can consume Curr LS indexes without Parent Index edit.
                i_parent
            }
        } else {
            // Let my children manage my rankings.
            let mut j_curr = 0;
            for son in self.sons.values() {
                print_with_offset(level, format!("> valuta figlio"));
                // i_parent = son.dump_onto_wbsa_deep(wbsa, this_real_rankings, i_parent, level + 1);
                j_curr = son.dump_onto_wbsa_deep(wbsa, sa, this_real_rankings, j_curr, level + 1);
            }
            print_with_offset(level, format!("> non dovrei sbufferare gli altri?"));
            // Managing the rest of my rankings myself.
            while j_curr < this_real_rankings.len() {
                // +
                sa.push(this_real_rankings[j_curr]);
                print_with_offset(level, format!("**[C]** {}", this_real_rankings[j_curr]));
                // -

                j_curr += 1;
            }
            i_parent
        }

        // Then, solve relationship with parent.
        // let min = if let Some(x) = self.min_father {x} else {};

        /*let mut result: Vec<usize> = Vec::new();

        if let Some(min_father) = self.min_father {
            // First Parent elements if present, then Child.
            if min_father > 0 {
                result.extend(&parent_real_rankings[0..min_father]);
            }
            result.extend(this_real_rankings);
        } else {
            // First Parent element, then Child.
            result.extend(parent_real_rankings);
            result.extend(this_real_rankings);
            // TODO: Can we ignore Max Father in this case?
        }
        print_with_offset(level, format!("  -> {:?}", result));*/
    }*/
    fn get_common_prefix_partition(&mut self, wbsa: &mut Vec<usize>) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        /*println!("\nNode: ");
        println!("{}", self.label);*/

        let common = self.get_real_rankings(wbsa);
        /*println!("common: ");
        println!("{:?}", common);*/

        if self.sons.is_empty() {
            result.extend(common);
            println!(
                "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                self.label,
                self.min_father,
                self.max_father,
                self.get_real_rankings(wbsa),
                result
            );
            /*println!("result: ");
            println!("{:?}", result);*/
            return result;
        }

        let mut position = 0;
        for son in self.sons.values_mut() {
            let temp = son.get_common_prefix_partition(wbsa);
            if let Some(min_father) = son.min_father {
                println!("Here self={} and son={}", self.label, son.label);
                if min_father >= position {
                    result.extend(&common[position..min_father]);
                }
                result.extend(temp);
                if let Some(max_father) = son.max_father {
                    position = max_father;
                } else {
                    position = min_father;
                }
            } else {
                // Min Father is None.
                result.extend(&common[position..]);
                result.extend(temp);
                position = common.len();
            }
        }
        result.extend(&common[position..]);
        println!(
            "Node {} (m={:?}, M={:?}) {:?} => {:?}",
            self.label,
            self.min_father,
            self.max_father,
            self.get_real_rankings(wbsa),
            result
        );
        /*println!("result: ");
        println!("{:?}", result);*/
        result
    }
    fn get_real_rankings(&self, wbsa: &Vec<usize>) -> Vec<usize> {
        if let Some(rankings) = &self.rankings_forced {
            // FIXME: Avoid cloning
            rankings.clone()
        } else {
            self.get_rankings(wbsa).to_vec()
        }
    }
    pub fn shrink_bottom_up(
        &mut self,
        wbsa: &mut Vec<usize>,
        depths: &mut Vec<usize>,
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

        // First, we Shrink the Children Nodes.
        for (_, son) in &mut self.sons {
            son.shrink_bottom_up(wbsa, depths, src, icfl_indexes, is_custom_vec, factor_list);
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

        // Children are Shrunk. Now their Rankings must be merged into their Parent Rankings.
        if self.get_rankings_count() == 0 {
            // This is a Bridge Node, so its Rankings are simply its Children Rankings in order.

            // TODO: Demonstrate this
            self.wbsa_q = self.get_max_buff_index_right_excl_from_righted_child();
            self.shrunk = true;

            self.sons.clear();

            // println!(" > \"bridge\" node {} fused with sons", self.label);
            return;
        }

        // Here, Parent Rankings have to accept Children Rankings. Where do we place them?
        let sons = &self.sons.values().collect::<Vec<_>>();

        // Pre-dimensioning the auxiliary memory for new Node's Rankings calculation
        let mut children_rankings_count = 0;
        for son in sons {
            children_rankings_count += son.get_rankings_count();
        }
        let mut result = Vec::with_capacity(self.get_rankings_count() + children_rankings_count);

        let mut i_father_index = self.get_buff_index_left();
        for son in sons {
            // Start by comparing Father Suffixes (using the length of this son, if
            // possible) and putting first the ones that are < Child Suffix.
            let child_node_height = son.suffix_len;
            let child_ls_length_height =
                son.get_string_from_first_ranking_with_length(wbsa, src, child_node_height);
            println!(
                " > merge father={} {:?} with child={} {:?}",
                self.label,
                self.get_rankings(wbsa),
                son.label,
                son.get_rankings(wbsa),
            );

            println!("   > phase 1: first father's smaller than child");
            while i_father_index < self.get_buff_index_right_excl() {
                let curr_father_ls_index = wbsa[i_father_index];
                let curr_father_ls = &src[curr_father_ls_index
                    ..usize::min(curr_father_ls_index + child_node_height, src.len())];

                // Comparing strings.
                if curr_father_ls < child_ls_length_height {
                    result.push(curr_father_ls_index);
                    println!("     > father ls index {curr_father_ls_index} added first");
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

            println!(
                "   > phase 2: window comparing via \"RULES\" from {}",
                wbsa[i_father_index]
            );
            // let son_first_ranking = wbsa[son.get_buff_index_left()];
            // let son_first_ranking_depth = depths[son_first_ranking];
            let mut max_i_father_index = i_father_index;
            while max_i_father_index < self.get_buff_index_right_excl() {
                let curr_father_ls_index = wbsa[max_i_father_index];
                let curr_father_ls = &src[curr_father_ls_index
                    ..usize::min(curr_father_ls_index + child_node_height, src.len())];

                // Comparing strings.
                if curr_father_ls > child_ls_length_height {
                    // Found Father Suffix that is > Curr. Child Suffix.
                    // println!("     > break because: {curr_father_ls} > {child_ls_length_height}");
                    break;
                } else {
                    max_i_father_index += 1;
                    // println!("     > incr. MAX_FATHER, comp. fail: {curr_father_ls} > {child_ls_length_height}   /   {}", son_first_ranking_depth);
                }
            }
            println!("     > indx [{}, {})", i_father_index, max_i_father_index);
            println!(
                "       > vals {:?}",
                &wbsa[i_father_index..max_i_father_index]
            );

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
                let child_suffix_len = child_node_height;
                let result_rules = Self::rules_safe(
                    curr_father_ls_index,
                    curr_child_ls_index,
                    child_suffix_len,
                    src,
                    &icfl_indexes,
                    &is_custom_vec,
                    &factor_list,
                );
                if !result_rules {
                    println!(
                        "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                        &src
                            [curr_father_ls_index..curr_father_ls_index + child_suffix_len], curr_father_ls_index, &src
                            [curr_child_ls_index..curr_child_ls_index + child_suffix_len], curr_child_ls_index, child_suffix_len
                    );
                    result.push(curr_father_ls_index);
                    i_father_index += 1;
                } else {
                    println!(
                        "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: son wins",
                        &src
                            [curr_father_ls_index..curr_father_ls_index + child_suffix_len], curr_father_ls_index, &src
                            [curr_child_ls_index..curr_child_ls_index + child_suffix_len], curr_child_ls_index, child_suffix_len
                    );
                    result.push(curr_child_ls_index);
                    j_child_index += 1;
                }
            }
            // Ok, we first take all Child Suffixes left, then continue to insert all
            // Father Suffixes left.
            println!("   > phase 3: then the last father's");
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
        println!("   > done with result={:?}", self.get_rankings(wbsa));
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
    fn rules_safe(
        x: usize,
        y: usize,
        child_offset: usize,
        src: &str,
        icfl_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) -> bool {
        let cmp1_father = &src[x + child_offset..];
        let cmp2_child = &src[y + child_offset..];
        let mut oracle;
        if cmp1_father < cmp2_child {
            oracle = false; // Father first.
        } else {
            oracle = true; // Child first.
        }
        let given = Self::rules(
            x,
            y,
            child_offset,
            src,
            icfl_list,
            is_custom_vec,
            factor_list,
        );

        // Debug only.
        /*if given != oracle {
            println!(
                " RULES: x={x:2}, y={y:2}, offset={child_offset} => {oracle}, BUT GIVEN WRONG!"
            );
        } else {
            // println!(" RULES: x={x:2}, y={y:2}, offset={child_offset} => {oracle}");
        }

        // oracle*/
        given
    }
}
fn print_with_offset(level: usize, str: String) {
    // TODO: Move from here
    println!("{} {}", "  ".repeat(level), str);
}
