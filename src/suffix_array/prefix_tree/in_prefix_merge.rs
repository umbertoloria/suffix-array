use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::new_tree::{Tree, TreeNode};
use crate::suffix_array::prefix_trie::rules::rules_safe;
use std::cell::Ref;

pub struct IPMergeParams<'a> {
    pub str: &'a str,
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
            let child_node = self.get_node(child_node_id).borrow();
            let child_node_cpp = self.in_prefix_merge_children_and_get_common_prefix_partition(
                &child_node,
                &child_node.rankings,
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
        self_node: &Ref<TreeNode<'a>>,
        self_rankings: &Vec<usize>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> Vec<usize> {
        // TODO: Avoid using auxiliary memory
        let mut result_cpp = Vec::new();
        let mut position = 0;

        for &(_, child_node_id) in &self_node.children {
            let child_node = self.get_node(child_node_id).borrow();
            let (
                //
                child_cpp,
                child_node_min_father,
                child_node_max_father,
            ) = self.in_prefix_merge_deep(
                &child_node,
                &child_node.rankings,
                self_rankings,
                ip_merge_params,
                monitor,
                verbose,
            );

            // Add all Self Node's Rankings.
            if let Some(min_father) = child_node_min_father {
                if verbose {
                    // Unfortunately, we don't have "self_node_id" :(
                    println!("Here self=?? and child={child_node_id}");
                }

                // There is a Min Father, so use all Self Node's Rankings remained (from "position")
                // until "min_father" position (if there are some).
                if position < min_father {
                    // There are some Self Node's Rankings from "position" to "min_father".
                    result_cpp.extend(&self_rankings[position..min_father]);
                    position = min_father;
                }
                if let Some(max_father) = child_node_max_father {
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
                result_cpp.extend(&self_rankings[position..]);
                position = self_rankings.len();
            }

            // Add all Child Node's Rankings.
            result_cpp.extend(child_cpp);
        }

        result_cpp.extend(&self_rankings[position..]);

        result_cpp
    }
    fn in_prefix_merge_deep(
        &self,
        self_node: &Ref<TreeNode<'a>>,
        self_rankings: &Vec<usize>,
        parent_rankings: &Vec<usize>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> (Vec<usize>, Option<usize>, Option<usize>) {
        let str = ip_merge_params.str;

        // Compare This Node's Rankings with Parent Node's Rankings.
        let this_first_ls_index = self_rankings[0];
        let this_ls_length = self_node.suffix_len;
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
            let parent_first_ls_index = parent_rankings[0];
            // Should use "parent_node.suffix_len".
            // let parent_ls_length = depths[parent_first_ls_index];
            // let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];
            println!(
                "Compare parent ({}) {:?} with curr ({}) {:?}",
                parent_first_ls_index, parent_rankings, this_ls, self_rankings
            );
        }

        // IN-PREFIX MERGE RANKINGS
        let mut self_node_min_father = None;
        let mut self_node_max_father = None;
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
                self_node_min_father = Some(i_parent);
                break;
            }
            i_parent += 1;
        }

        // TODO: Avoid cloning Rankings into auxiliary memory
        let mut new_self_rankings = Vec::new();
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
                self_node_max_father = Some(max_father);

                i_parent = self_node_min_father.unwrap();

                // Now, the Window for Comparing Rankings using "RULES" is the following:
                // - starts from "i_parent", included, and
                // - ends with "max_father", excluded.
                if verbose {
                    println!("   > start comparing, window=[{},{})", i_parent, max_father);
                }

                let mut j_this = 0;
                while i_parent < max_father && j_this < self_rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_this_ls_index = self_rankings[j_this];
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
                        new_self_rankings.push(curr_parent_ls_index);
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
                        new_self_rankings.push(curr_this_ls_index);
                        j_this += 1;
                    }
                }
                if j_this < self_rankings.len() {
                    // Enters in following while.
                } else {
                    if verbose {
                        println!("     > no child rankings left to add");
                    }
                }
                while j_this < self_rankings.len() {
                    let curr_this_ls_index = self_rankings[j_this];
                    if verbose {
                        let child_offset = self_node.suffix_len;
                        println!(
                            "     > adding child=\"{}\" [{}], child.suff.len={}",
                            &str[curr_this_ls_index..curr_this_ls_index + child_offset],
                            curr_this_ls_index,
                            child_offset
                        );
                    }
                    new_self_rankings.push(curr_this_ls_index);
                    j_this += 1;
                }
                if let Some(max_father) = self_node_max_father {
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
                        new_self_rankings.push(curr_parent_ls_index);
                        i_parent += 1;
                    }
                } else {
                    if verbose {
                        println!("     > no parent rankings left to add");
                    }
                }

                // Avoid editing Node Rankings. Instead, we use "new_self_rankings".
                // self_rankings.clear();
                // self_rankings.append(&mut new_rankings);
            }
        }

        // Now it's your turn to be the parent.
        let result_cpp = self.in_prefix_merge_children_and_get_common_prefix_partition(
            self_node,
            if new_self_rankings.is_empty() {
                self_rankings
            } else {
                &new_self_rankings
            },
            ip_merge_params,
            monitor,
            verbose,
        );

        (result_cpp, self_node_min_father, self_node_max_father)
    }
}
