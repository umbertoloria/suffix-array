use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::rules::rules_safe;
use crate::suffix_array::prefix_tree::tree::{Tree, TreeNode};
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
        let mut suffix_array = Vec::with_capacity(str_length);

        let root_node = self.get_root().borrow();
        for &(_, child_node_id) in &root_node.children {
            // Visiting from all First Layer Nodes to all Leafs (avoiding Root Node).
            let child_node = self.get_node(child_node_id).borrow();

            self.in_prefix_merge_and_get_common_prefix_partition(
                &child_node,
                &child_node.rankings,
                ip_merge_params,
                monitor,
                verbose,
                &mut suffix_array,
            );
        }

        suffix_array
    }
    fn in_prefix_merge_and_get_common_prefix_partition(
        &self,
        self_node: &Ref<TreeNode<'a>>,
        self_rankings: &Vec<usize>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
        suffix_array: &mut Vec<usize>,
    ) {
        let mut position = 0;

        for &(_, child_node_id) in &self_node.children {
            let child_node = self.get_node(child_node_id).borrow();

            // CHILD NODE: Window for Comparing Rankings using "RULES"
            let (
                //
                child_node_min_father,
                child_node_max_father,
                child_new_rankings,
            ) = self.in_prefix_merge_get_min_max_and_new_rankings(
                child_node.suffix_len,
                &child_node.rankings,
                self_rankings, // As Parent Node's Rankings.
                ip_merge_params,
                monitor,
                verbose,
            );

            // SELF COMMON PREFIX PARTITION: Self Node's Rankings from left
            if let Some(min_father) = child_node_min_father {
                if verbose {
                    // Unfortunately, we don't have "self_node_id" :(
                    println!("Here self=?? and child={child_node_id}");
                }

                // There is a Min Father, so use all Self Node's Rankings remained (from "position")
                // until "min_father" position (if there are some).
                if position < min_father {
                    // There are some Self Node's Rankings from "position" to "min_father".

                    suffix_array.extend(&self_rankings[position..min_father]);
                    // result_cpp.extend(&self_rankings[position..min_father]);
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

                suffix_array.extend(&self_rankings[position..]);
                // result_cpp.extend(&self_rankings[position..]);
                position = self_rankings.len();
            }

            // SELF COMMON PREFIX PARTITION: Child Node's Rankings
            if let Some(child_new_rankings) = child_new_rankings {
                self.in_prefix_merge_and_get_common_prefix_partition(
                    &child_node,
                    &child_new_rankings,
                    ip_merge_params,
                    monitor,
                    verbose,
                    suffix_array,
                )
            } else {
                self.in_prefix_merge_and_get_common_prefix_partition(
                    &child_node,
                    &child_node.rankings,
                    ip_merge_params,
                    monitor,
                    verbose,
                    suffix_array,
                )
            };
            // result_cpp.extend(&child_cpp);
        }

        // SELF COMMON PREFIX PARTITION: Self Node's Rankings remained
        suffix_array.extend(&self_rankings[position..]);
        // result_cpp.extend(&self_rankings[position..]);
    }
    fn in_prefix_merge_get_min_max_and_new_rankings(
        &self,
        self_node_suffix_len: usize,
        self_rankings: &Vec<usize>,
        parent_rankings: &Vec<usize>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> (
        Option<usize>,      // Min Father
        Option<usize>,      // Max Father
        Option<Vec<usize>>, // New Self Node's Rankings
    ) {
        let str = ip_merge_params.str;

        // Compare This Node's Rankings with Parent Node's Rankings.
        let this_first_ls_index = self_rankings[0];
        let this_ls_length = self_node_suffix_len;
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
                break;
            }
            i_parent += 1;
        }
        if i_parent < parent_rankings.len() {
            // Go further.
        } else {
            // There's no "min_father".
            return (None, None, None);
        }

        // Assuming "i_parent < parent_rankings.len()".

        let min_father = i_parent;
        // From here, we have a "min_father" value. So it's true that there is at least one
        // Parent Ranking that have LS that is >= than Curr LS.

        // If Curr Parent Ranking has LS that is > than Curr LS then "max_father"=None. So there
        // would be no Window for Comparing Rankings using "RULES", since from here on all
        // Parent Rankings would have LSs that are > than Curr LS.

        // If Curr Parent Ranking has LS that is == than Curr LS then we would look for a Max Father
        // to set in order to have a Window for Comparing Rankings using "RULES".

        let curr_parent_ls_index = parent_rankings[i_parent];
        let curr_parent_ls = &str
            [curr_parent_ls_index..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
        // TODO: Monitor string compare
        if curr_parent_ls == this_ls {
            // Go further.
        } else {
            // Assuming "curr_parent_ls > this_ls".
            // This means "max_father"=None. There's no Window for Comparing Rankings using "RULES".

            return (Some(min_father), None, None);
        }

        // Assuming "curr_parent_ls == this_ls".

        // There is at least one Parent Ranking that is = to Curr LS. This means that there is a
        // Window for Comparing Rankings using "RULES" to create. So now we are looking for the
        // Max Father for closing this Window.
        i_parent += 1;
        while i_parent < parent_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            // TODO: Monitor string compare
            if curr_parent_ls == this_ls {
                // Ok. This Parent Ranking is = to Curr LS as well, so the Window is not closed yet.
            } else {
                // Found a Parent LS that is > Curr LS.
                break;
            }
            i_parent += 1;
        }
        let max_father = i_parent;
        i_parent = min_father;

        // Now, the Window for Comparing Rankings using "RULES" is the following:
        // - starts from "i_parent", included, and
        // - ends with "max_father", excluded.
        if verbose {
            println!("   > start comparing, window=[{},{})", i_parent, max_father);
        }

        // TODO: Avoid cloning Rankings into auxiliary memory
        let mut new_self_rankings = Vec::new();
        let mut j_this = 0;
        while i_parent < max_father && j_this < self_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_this_ls_index = self_rankings[j_this];
            let child_offset = self_node_suffix_len;
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
                    let curr_this_ls = &str[curr_this_ls_index..curr_this_ls_index + child_offset];
                    println!(
                        "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                        curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index,
                        child_offset,
                    );
                }
                new_self_rankings.push(curr_parent_ls_index);
                i_parent += 1;
            } else {
                if verbose {
                    let curr_parent_ls =
                        &str[curr_parent_ls_index..curr_parent_ls_index + child_offset];
                    let curr_this_ls = &str[curr_this_ls_index..curr_this_ls_index + child_offset];
                    println!(
                        "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
                        curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index,
                        child_offset,
                    );
                }
                new_self_rankings.push(curr_this_ls_index);
                j_this += 1;
            }
        }
        if j_this >= self_rankings.len() {
            if verbose {
                println!("     > no child rankings left to add");
            }
        }
        while j_this < self_rankings.len() {
            let curr_this_ls_index = self_rankings[j_this];
            if verbose {
                let child_offset = self_node_suffix_len;
                println!(
                    "     > adding child=\"{}\" [{}], child.suff.len={}",
                    &str[curr_this_ls_index..curr_this_ls_index + child_offset],
                    curr_this_ls_index,
                    child_offset,
                );
            }
            new_self_rankings.push(curr_this_ls_index);
            j_this += 1;
        }
        while i_parent < max_father {
            let curr_parent_ls_index = parent_rankings[i_parent];
            if verbose {
                let child_offset = self_node_suffix_len;
                println!(
                    "     > adding father=\"{}\" [{}], father.suff.len={}",
                    &str[curr_parent_ls_index..curr_parent_ls_index + child_offset],
                    curr_parent_ls_index,
                    child_offset,
                );
            }
            new_self_rankings.push(curr_parent_ls_index);
            i_parent += 1;
        }

        // From here, the *NEW* Self Node's Rankings are "new_self_rankings".
        (Some(min_father), Some(max_father), Some(new_self_rankings))
    }
}
