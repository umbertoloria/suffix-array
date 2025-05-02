use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_trie::prefix_trie::{PrefixTrie, PrefixTrieData};
use crate::suffix_array::prefix_trie::rules::rules_safe;
use crate::suffix_array::prefix_trie::tree_bank_min_max::TreeBankMinMax;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl<'a> PrefixTrie<'a> {
    pub fn in_prefix_merge(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        tree_bank_min_max: &mut TreeBankMinMax,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        match &mut self.data {
            PrefixTrieData::Leaf => {}
            PrefixTrieData::DirectChild((_, child_node)) => {
                child_node.in_prefix_merge_on_children(
                    str,
                    prog_sa,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    icfl_factor_list,
                    tree_bank_min_max,
                    compare_cache,
                    monitor,
                    verbose,
                );
            }
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    child_node.in_prefix_merge_on_children(
                        str,
                        prog_sa,
                        depths,
                        icfl_indexes,
                        is_custom_vec,
                        icfl_factor_list,
                        tree_bank_min_max,
                        compare_cache,
                        monitor,
                        verbose,
                    );
                }
            }
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    child_node.in_prefix_merge_on_children(
                        str,
                        prog_sa,
                        depths,
                        icfl_indexes,
                        is_custom_vec,
                        icfl_factor_list,
                        tree_bank_min_max,
                        compare_cache,
                        monitor,
                        verbose,
                    );
                }
            }
        }
    }
    fn in_prefix_merge_on_children(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        tree_bank_min_max: &mut TreeBankMinMax,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        let self_id = self.id;
        match &mut self.data {
            PrefixTrieData::Leaf => {}
            PrefixTrieData::DirectChild((_, child_node)) => {
                child_node.in_prefix_merge_deep(
                    str,
                    prog_sa,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    icfl_factor_list,
                    self_id,
                    tree_bank_min_max,
                    compare_cache,
                    monitor,
                    verbose,
                );
            }
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    child_node.in_prefix_merge_deep(
                        str,
                        prog_sa,
                        depths,
                        icfl_indexes,
                        is_custom_vec,
                        icfl_factor_list,
                        self_id,
                        tree_bank_min_max,
                        compare_cache,
                        monitor,
                        verbose,
                    );
                }
            }
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    child_node.in_prefix_merge_deep(
                        str,
                        prog_sa,
                        depths,
                        icfl_indexes,
                        is_custom_vec,
                        icfl_factor_list,
                        self_id,
                        tree_bank_min_max,
                        compare_cache,
                        monitor,
                        verbose,
                    );
                }
            }
        }
    }
    fn in_prefix_merge_deep(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        parent_index: usize,
        tree_bank_min_max: &mut TreeBankMinMax,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        // Compare This Node's Rankings with Parent Node's Rankings.
        let parent_rankings = prog_sa.get_rankings(parent_index);

        let this_rankings = prog_sa.get_rankings(self.id);
        let this_first_ls_index = this_rankings[0];
        let this_ls_length = depths[this_first_ls_index];
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
            let parent_first_ls_index = parent_rankings[0];
            let parent_ls_length = depths[parent_first_ls_index];
            let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];
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
                tree_bank_min_max.get_mut(self.id).min_father = Some(i_parent);
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
                        tree_bank_min_max.get_mut(self.id).max_father = Some(i_parent + 1);
                        i_parent += 1;
                    } else {
                        // Found a Parent LS that is > Curr LS.
                        break;
                    }
                }

                let self_node_min_max = tree_bank_min_max.get(self.id);
                i_parent = self_node_min_max.min_father.unwrap();
                let mut j_this = 0;

                let mut new_rankings = Vec::new();
                if let Some(max_father) = self_node_min_max.max_father {
                    if verbose {
                        println!("   > start comparing, window=[{},{})", i_parent, max_father);
                    }
                    while i_parent < max_father && j_this < this_rankings.len() {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        let curr_this_ls_index = this_rankings[j_this];
                        let child_offset = self.suffix_len; // Could be inline.
                        let result_rules = rules_safe(
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
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
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
                if let Some(max_father) = self_node_min_max.max_father {
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
                prog_sa.save_rankings_forced(self.id, new_rankings);
            }
        }

        // Now it's your turn to be the parent.
        self.in_prefix_merge_on_children(
            str,
            prog_sa,
            depths,
            icfl_indexes,
            is_custom_vec,
            icfl_factor_list,
            tree_bank_min_max,
            compare_cache,
            monitor,
            verbose,
        );
    }
}
