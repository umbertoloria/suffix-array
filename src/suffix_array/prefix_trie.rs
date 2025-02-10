use crate::suffix_array::chunking::get_max_size;
use crate::suffix_array::compare_cache::CompareCache;
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
    pub rankings_forced: Option<Vec<usize>>,
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
            rankings_forced: None,
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
    fn get_rankings<'a>(&self, wbsa: &'a Vec<usize>, wbsa_indexes: &WbsaIndexes) -> &'a [usize] {
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
            self.get_real_rankings(wbsa, wbsa_indexes),
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
    pub fn in_prefix_merge(
        &mut self,
        str: &str,
        wbsa: &mut Vec<usize>,
        wbsa_indexes: &mut WbsaIndexes,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        if self.suffix_len == 0 {
            // This is the Root Node.
            for son in self.sons.values_mut() {
                son.in_prefix_merge(
                    str,
                    wbsa,
                    wbsa_indexes,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    icfl_factor_list,
                    compare_cache,
                    monitor,
                    verbose,
                );
            }
            return;
        }

        if self.get_rankings_count(wbsa_indexes) == 0 {
            // This is a Bridge Node.
            for son in self.sons.values_mut() {
                son.in_prefix_merge(
                    str,
                    wbsa,
                    wbsa_indexes,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    icfl_factor_list,
                    compare_cache,
                    monitor,
                    verbose,
                );
            }
            return;
        }

        // Node with Rankings.
        let this_ranking = self.get_real_rankings(wbsa, wbsa_indexes);
        for son in self.sons.values_mut() {
            son.in_prefix_merge_deep(
                str,
                wbsa,
                wbsa_indexes,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                &this_ranking,
                compare_cache,
                monitor,
                verbose,
            );
        }
    }
    fn in_prefix_merge_deep(
        &mut self,
        str: &str,
        wbsa: &mut Vec<usize>,
        wbsa_indexes: &mut WbsaIndexes,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        parent_rankings: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        // Parent has to *ALWAYS* have rankings.

        if self.get_rankings_count(wbsa_indexes) == 0 {
            // This is a Bridge Node.
            for son in self.sons.values_mut() {
                son.in_prefix_merge_deep(
                    str,
                    wbsa,
                    wbsa_indexes,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    icfl_factor_list,
                    parent_rankings,
                    compare_cache,
                    monitor,
                    verbose,
                );
            }
            return;
        }

        // Compare this node's rankings with parent's rankings.
        let parent_first_ls_index = parent_rankings[0];
        let parent_ls_length = depths[parent_first_ls_index];
        let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];

        let this_rankings = self.get_real_rankings(wbsa, wbsa_indexes);
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
            // TODO: Monitor string compare
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
            // TODO: Monitor string compare
            if curr_parent_ls > this_ls {
                // This means "max_father"=None.
                // There's no Window for Comparing Rankings using "RULES".
            } else {
                while i_parent < parent_rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_parent_ls = &str[curr_parent_ls_index
                        ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
                    // TODO: Monitor string compare
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

                let mut new_rankings = Vec::new();
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
                            icfl_indexes,
                            &is_custom_vec,
                            &icfl_factor_list,
                            compare_cache,
                            monitor,
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
                            new_rankings.push(curr_parent_ls_index);
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
                            new_rankings.push(curr_this_ls_index);
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
                    new_rankings.push(curr_this_ls_index);
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
                        new_rankings.push(curr_parent_ls_index);
                        i_parent += 1;
                    }
                } else {
                    if verbose {
                        println!("     > no parent rankings left to add");
                    }
                }
                self.rankings_forced = Some(new_rankings);
            }
        }

        // Now it's your turn to be the parent.
        let this_rankings = self.get_real_rankings(wbsa, wbsa_indexes);
        for son in self.sons.values_mut() {
            son.in_prefix_merge_deep(
                str,
                wbsa,
                wbsa_indexes,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                &this_rankings,
                compare_cache,
                monitor,
                verbose,
            );
        }
    }
    pub fn get_real_rankings(&self, wbsa: &Vec<usize>, wbsa_indexes: &WbsaIndexes) -> Vec<usize> {
        if let Some(rankings) = &self.rankings_forced {
            // FIXME: Avoid cloning
            rankings.clone()
        } else {
            self.get_rankings(wbsa, wbsa_indexes).to_vec()
        }
    }
    fn rules(
        x: usize,
        y: usize,
        child_offset: usize,
        src: &str,
        icfl_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
    ) -> bool {
        let icfl_list_size = icfl_list.len();
        if is_custom_vec[x] && is_custom_vec[y] {
            monitor.new_compare_of_two_ls_in_custom_factors();
            monitor.new_compare_using_actual_string_compare();
            compare_cache.compare_1_before_2(
                //
                src,
                y + child_offset,
                x + child_offset,
            )
            /*let cmp1 = &src[y + child_offset..];
            let cmp2 = &src[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
        } else if is_custom_vec[x] {
            monitor.new_compare_one_ls_in_custom_factor();
            if icfl_factor_list[x] <= icfl_factor_list[y] {
                monitor.new_compare_using_rules();
                if y >= icfl_list[icfl_list_size - 1] {
                    true
                } else {
                    false
                }
            } else {
                monitor.new_compare_using_actual_string_compare();
                compare_cache.compare_1_before_2(
                    //
                    src,
                    y + child_offset,
                    x + child_offset,
                )
                /*let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }*/
            }
        } else if is_custom_vec[y] {
            monitor.new_compare_one_ls_in_custom_factor();
            if icfl_factor_list[y] <= icfl_factor_list[x] {
                monitor.new_compare_using_rules();
                if x >= icfl_list[icfl_list_size - 1] {
                    false
                } else {
                    true
                }
            } else {
                monitor.new_compare_using_actual_string_compare();
                compare_cache.compare_1_before_2(
                    //
                    src,
                    y + child_offset,
                    x + child_offset,
                )
                /*let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }*/
            }
        } else if x >= icfl_list[icfl_list_size - 1] && y >= icfl_list[icfl_list_size - 1] {
            monitor.new_compare_using_rules();
            false
        } else if icfl_factor_list[x] == icfl_factor_list[y] {
            monitor.new_compare_using_rules();
            true
        } else {
            if x >= icfl_list[icfl_list_size - 1] {
                monitor.new_compare_using_rules();
                false
            } else if y >= icfl_list[icfl_list_size - 1] {
                monitor.new_compare_using_actual_string_compare();
                compare_cache.compare_1_before_2(
                    //
                    src,
                    y + child_offset,
                    x + child_offset,
                )
                /*let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }*/
            } else {
                if x > y {
                    monitor.new_compare_using_rules();
                    true
                } else {
                    monitor.new_compare_using_actual_string_compare();
                    compare_cache.compare_1_before_2(
                        //
                        src,
                        y + child_offset,
                        x + child_offset,
                    )
                    /*let cmp1 = &src[y + child_offset..];
                    let cmp2 = &src[x + child_offset..];
                    if cmp1 < cmp2 {
                        true
                    } else {
                        false
                    }*/
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
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
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
                icfl_factor_list,
                compare_cache,
                monitor,
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
                icfl_factor_list,
                compare_cache,
                monitor,
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
