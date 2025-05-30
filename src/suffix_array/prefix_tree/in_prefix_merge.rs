use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::new_tree::{Tree, TreeNode};
use crate::suffix_array::prefix_trie::rules::rules_safe;
use std::cell::RefMut;

pub struct IPMergeParams<'a> {
    pub str: &'a str,
    pub depths: &'a Vec<usize>,
    pub icfl_indexes: &'a Vec<usize>,
    pub is_custom_vec: &'a Vec<bool>,
    pub icfl_factor_list: &'a Vec<usize>,
    pub compare_cache: &'a mut CompareCache,
}

impl<'a> Tree<'a> {
    pub fn in_prefix_merge(
        &self,
        str_length: usize,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> Vec<usize> {
        let mut sa = Vec::with_capacity(str_length);
        let root_node = self.get_root().borrow();
        for &(_, child_node_id) in &root_node.children {
            // Visiting from all First Layer Nodes to all Leafs (avoiding Root Node).
            let mut child_node = self.get_node(child_node_id).borrow_mut();
            let child_node_cpp = self.in_prefix_merge_children_and_get_common_prefix_partition(
                &mut child_node,
                ip_merge_params,
                monitor,
                verbose,
            );
            sa.extend(child_node_cpp);
        }
        sa
    }
    fn in_prefix_merge_children_and_get_common_prefix_partition(
        &self,
        self_node: &mut RefMut<TreeNode<'a>>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result_cpp = Vec::new();
        let mut position = 0;

        for &(_, child_node_id) in &self_node.children {
            let mut child_node = self.get_node(child_node_id).borrow_mut();
            let child_cpp = self.in_prefix_merge_deep(
                &mut child_node,
                &self_node,
                ip_merge_params,
                monitor,
                verbose,
            );

            // Add all Self Node's Rankings.
            if let Some(min_father) = child_node.min {
                if verbose {
                    // Unfortunately, we don't have "self_node_id" :(
                    println!("Here self=?? and child={child_node_id}");
                }

                // There is a Min Father, so use all Self Node's Rankings remained (from "position")
                // until "min_father" position (if there are some).
                if position < min_father {
                    // There are some Self Node's Rankings from "position" to "min_father".
                    result_cpp.extend(&self_node.rankings[position..min_father]);
                    position = min_father;
                }
                if let Some(max_father) = child_node.max {
                    // Here, there is both a Min Father and a Max Father, this means that there
                    // exist a Window for Comparing Rankings using "RULES". This means that all
                    // Self Node's Rankings from Min Father to Max Father should be ignored since
                    // they are transformed into "Inherited Rankings" (during In-prefix Merge Phase)
                    // now, so they are inside as Children's Rankings and will be dealt with later
                    // (when these Children Nodes become Self Nodes for this procedure).
                    position = max_father;
                }
            } else {
                // Min Father is None. There's no Window for Comparing Rankings using "RULES". This
                // means that:
                // all Self Node's Rankings have LSs that are < than all Child Node's Rankings.
                // So we take firstly Self Node's Rankings, and then all the Child Node's Rankings.
                result_cpp.extend(&self_node.rankings[position..]);
                position = self_node.rankings.len();
            }

            // Add all Child Node's Rankings.
            result_cpp.extend(child_cpp);
        }

        result_cpp.extend(&self_node.rankings[position..]);

        result_cpp
    }
    fn in_prefix_merge_deep(
        &self,
        self_node: &mut RefMut<TreeNode<'a>>,
        parent_node: &RefMut<TreeNode<'a>>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> Vec<usize> {
        let str = ip_merge_params.str;
        let depths = ip_merge_params.depths;

        // Compare This Node's Rankings with Parent Node's Rankings.
        let parent_rankings = &parent_node.rankings;

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
                i_parent += 1;
                while i_parent < parent_rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_parent_ls = &str[curr_parent_ls_index
                        ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
                    // TODO: Monitor string compare
                    if curr_parent_ls == this_ls {
                        // Ok. This Parent Ranking is = than Curr LS as well. So the Window is not
                        // closed yet.
                    } else {
                        // Found a Parent LS that is > Curr LS.
                        break;
                    }
                    i_parent += 1;
                }
                let max_father = i_parent;
                self_node.max = Some(max_father);

                i_parent = self_node.min.unwrap();

                // Now, the Window for Comparing Rankings using "RULES" is the following:
                // - starts from "i_parent", included, and
                // - ends with "max_father", excluded.
                if verbose {
                    println!("   > start comparing, window=[{},{})", i_parent, max_father);
                }

                // TODO: Avoid cloning Rankings (that live in Parent Rankings) into Child Rankings
                let mut new_rankings = Vec::new();
                let mut j_this = 0;
                while i_parent < max_father && j_this < self_node.rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_this_ls_index = self_node.rankings[j_this];
                    let child_offset = self_node.suffix_len;
                    let result_rules = rules_safe(
                        curr_parent_ls_index,
                        curr_this_ls_index,
                        child_offset,
                        ip_merge_params,
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
        self.in_prefix_merge_children_and_get_common_prefix_partition(
            self_node,
            ip_merge_params,
            monitor,
            verbose,
        )
    }
}
