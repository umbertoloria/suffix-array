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
    }
    pub fn print_with_wbsa(&self, tabs_offset: usize, prefix: String, wbsa: &Vec<usize>) {
        println!(
            "{}\"{}\" {:?}   min={}, MAX={}",
            "\t".repeat(tabs_offset),
            prefix,
            // self.label,
            self.get_real_rankings(wbsa),
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
        let this_ranking = self.get_real_rankings(wbsa);
        for son in self.sons.values_mut() {
            son.in_prefix_merge_deep(
                str,
                wbsa,
                depths,
                icfl_indexes,
                is_custom_vec,
                factor_list,
                &this_ranking,
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
        parent_rankings: &Vec<usize>,
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
                    parent_rankings,
                    verbose,
                );
            }
            return;
        }

        // Compare this node's rankings with parent's rankings.
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
                        let child_offset = self.suffix_len; // Could be inline.
                        let result_rules = Self::rules_safe(
                            curr_parent_ls_index,
                            curr_this_ls_index,
                            child_offset,
                            str,
                            &icfl_indexes,
                            &is_custom_vec,
                            &factor_list,
                            false,
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
                if j_this < this_rankings.len() {
                    // Enters in following while.
                } else {
                    if verbose {
                        println!("     > no child rankings left to add");
                    }
                }
                while j_this < this_rankings.len() {
                    let curr_this_ls_index = this_rankings[j_this];
                    let child_offset = self.suffix_len; // Could be inline.
                    if verbose {
                        println!(
                            "     > adding child=\"{}\" [{}], child.suff.len={}",
                            &str[curr_this_ls_index..curr_this_ls_index + child_offset],
                            curr_this_ls_index,
                            child_offset
                        );
                    }
                    result.push(curr_this_ls_index);
                    j_this += 1;
                }
                if let Some(max_father) = self.max_father {
                    while i_parent < max_father {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        let child_offset = self.suffix_len; // Could be inline.
                        if verbose {
                            println!(
                                "     > adding father=\"{}\" [{}], father.suff.len={}",
                                &str[curr_parent_ls_index..curr_parent_ls_index + child_offset],
                                curr_parent_ls_index,
                                child_offset
                            );
                        }
                        result.push(curr_parent_ls_index);
                        i_parent += 1;
                    }
                } else {
                    if verbose {
                        println!("     > no parent rankings left to add");
                    }
                }
                self.rankings_forced = Some(result);
            }
        }

        // Now it's your turn to be the parent.
        let this_rankings = self.get_real_rankings(wbsa);
        for son in self.sons.values_mut() {
            son.in_prefix_merge_deep(
                str,
                wbsa,
                depths,
                icfl_indexes,
                is_custom_vec,
                factor_list,
                &this_rankings,
                verbose,
            );
        }
    }
    pub fn get_real_rankings(&self, wbsa: &Vec<usize>) -> Vec<usize> {
        if let Some(rankings) = &self.rankings_forced {
            // FIXME: Avoid cloning
            rankings.clone()
        } else {
            self.get_rankings(wbsa).to_vec()
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
    fn rules_safe(
        x: usize,
        y: usize,
        child_offset: usize,
        src: &str,
        icfl_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
        slow_check: bool,
    ) -> bool {
        if !slow_check {
            Self::rules(
                x,
                y,
                child_offset,
                src,
                icfl_list,
                is_custom_vec,
                factor_list,
            )
        } else {
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
            if given != oracle {
                println!(
                    " RULES: x={x:2}, y={y:2}, offset={child_offset} => {oracle}, BUT GIVEN WRONG!"
                );
            } else {
                // println!(" RULES: x={x:2}, y={y:2}, offset={child_offset} => {oracle}");
            }
            oracle
        }
    }
}
fn print_with_offset(level: usize, str: String) {
    // TODO: Move from here
    println!("{} {}", "  ".repeat(level), str);
}
