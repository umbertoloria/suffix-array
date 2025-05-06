use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::new_tree::{Tree, TreeNode};
use crate::suffix_array::prefix_trie::rules::rules_safe;
use std::cell::RefMut;

impl<'a> Tree<'a> {
    pub fn in_prefix_merge(
        &self,
        str: &str,
        depths: &Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        for &(_, first_layer_node_id) in &self.get_root().borrow().children {
            // Visiting from all First Layer Nodes to all Leafs (avoiding Root Node).
            let first_layer_node = self.get_node(first_layer_node_id).borrow_mut();

            for &(_, second_layer_node_id) in &first_layer_node.children {
                // All Second Layer Nodes.
                self.in_prefix_merge_deep(
                    second_layer_node_id,
                    &first_layer_node,
                    str,
                    depths,
                    icfl_indexes,
                    is_custom_vec,
                    icfl_factor_list,
                    compare_cache,
                    monitor,
                    verbose,
                );
            }
        }
    }
    fn in_prefix_merge_deep(
        &self,
        self_node_id: usize,
        parent_node: &RefMut<TreeNode<'a>>,
        str: &str,
        depths: &Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        // Compare This Node's Rankings with Parent Node's Rankings.
        let parent_rankings = &parent_node.rankings;

        let mut self_node = self.get_node(self_node_id).borrow_mut();
        let this_first_ls_index = self_node.rankings[0];
        // TODO: It seems that "depths[*]" is always achievable using "*_node.suffix_len"
        let this_ls_length = depths[this_first_ls_index];
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
            let parent_first_ls_index = parent_rankings[0];
            let parent_ls_length = depths[parent_first_ls_index];
            let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];
            println!(
                "Compare parent ({}) {:?} with curr ({}) {:?}",
                parent_ls, parent_rankings, this_ls, self_node.rankings
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
                // Ok. All Parent Rankings from left to here have LSs that are < than Curr LS.
            } else {
                // Found a Parent LS that is >= Curr LS.
                self_node.min = Some(i_parent);
                break;
            }
            i_parent += 1;
        }
        if i_parent < parent_rankings.len() {
            // From here, we have a "min_father" value. So it's true that there is at least one
            // Parent Ranking that have LS that is >= than Curr LS.

            // If Curr Parent Ranking has LS that is > than Curr LS then "max_father"=None. So there
            // would be no Window for Comparing Rankings using "RULES", since from here on all
            // Parent Rankings would have LSs that are > than Curr LS.

            // If Curr Parent Ranking has LS that is == than Curr LS then we would look for a
            // Max Father to set in order to have a Window for Comparing Rankings using "RULES".

            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            /*if curr_parent_ls > this_ls {
                // This means "max_father"=None.
                // There's no Window for Comparing Rankings using "RULES".
            }*/
            // TODO: Monitor string compare
            if curr_parent_ls == this_ls {
                // There is at least one Parent Ranking that is == to Curr LS. This means that there
                // is a Window for Comparing Rankings using "RULES" to create. So now we are looking
                // for the Max Father for closing this Window.
                while i_parent < parent_rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_parent_ls = &str[curr_parent_ls_index
                        ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
                    // TODO: Monitor string compare
                    if curr_parent_ls == this_ls {
                        // Ok. This Parent Ranking is = than Curr LS as well. So the Window is not
                        // closed yet.
                        self_node.max = Some(i_parent + 1);
                    } else {
                        // Found a Parent LS that is > Curr LS.
                        break;
                    }
                    i_parent += 1;
                }

                i_parent = self_node.min.unwrap();
                let mut j_this = 0;

                let mut new_rankings = Vec::new();
                if let Some(max_father) = self_node.max {
                    if verbose {
                        println!("   > start comparing, window=[{},{})", i_parent, max_father);
                    }
                    while i_parent < max_father && j_this < self_node.rankings.len() {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        let curr_this_ls_index = self_node.rankings[j_this];
                        let child_offset = self_node.suffix_len;
                        let result_rules = rules_safe(
                            curr_parent_ls_index,
                            curr_this_ls_index,
                            child_offset,
                            str,
                            icfl_indexes,
                            is_custom_vec,
                            icfl_factor_list,
                            compare_cache,
                            monitor,
                            false,
                        );
                        if !result_rules {
                            if verbose {
                                let curr_parent_ls =
                                    &str[curr_parent_ls_index..curr_parent_ls_index + child_offset];
                                let curr_this_ls =
                                    &str[curr_this_ls_index..curr_this_ls_index + child_offset];
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                                    curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index, child_offset,
                                );
                            }
                            new_rankings.push(curr_parent_ls_index);
                            i_parent += 1;
                        } else {
                            if verbose {
                                let curr_parent_ls =
                                    &str[curr_parent_ls_index..curr_parent_ls_index + child_offset];
                                let curr_this_ls =
                                    &str[curr_this_ls_index..curr_this_ls_index + child_offset];
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
                                    curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index, child_offset,
                                );
                            }
                            new_rankings.push(curr_this_ls_index);
                            j_this += 1;
                        }
                    }
                }
                if j_this < self_node.rankings.len() {
                    // Enters in following while.
                } else {
                    if verbose {
                        println!("     > no child rankings left to add");
                    }
                }
                while j_this < self_node.rankings.len() {
                    let curr_this_ls_index = self_node.rankings[j_this];
                    if verbose {
                        let child_offset = self_node.suffix_len;
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
                if let Some(max_father) = self_node.max {
                    while i_parent < max_father {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        if verbose {
                            let child_offset = self_node.suffix_len;
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
                self_node.rankings.clear();
                self_node.rankings.append(&mut new_rankings);
            }
        }

        // Now it's your turn to be the parent.
        for &(_, child_node_id) in &self_node.children {
            self.in_prefix_merge_deep(
                child_node_id,
                &self_node,
                str,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                compare_cache,
                monitor,
                verbose,
            );
        }
    }
}
