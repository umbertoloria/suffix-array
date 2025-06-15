use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::rules::rules_safe;
use crate::suffix_array::prefix_tree::tree::{Tree, TreeNode};
use std::cell::Ref;

pub struct IPMergeParams<'a> {
    pub str: &'a str,
    pub icfl_indexes: &'a Vec<usize>,
    pub idx_to_is_custom: &'a Vec<bool>,
    pub idx_to_icfl_factor: &'a Vec<usize>,
}

impl<'a> Tree<'a> {
    pub fn in_prefix_merge_and_common_prefix_partition(
        &self,
        str_length: usize,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
    ) -> Vec<usize> {
        let mut suffix_array = Vec::with_capacity(str_length);

        for (_, child_node) in &self.root.children {
            // Visiting from all First Layer Nodes to all Leafs (avoiding Root Node).
            self.in_prefix_merge_and_get_common_prefix_partition(
                child_node,
                &child_node.rankings,
                ip_merge_params,
                monitor,
                &mut suffix_array,
            );
        }

        suffix_array
    }
    fn in_prefix_merge_and_get_common_prefix_partition(
        &self,
        self_node: &TreeNode<'a>,
        self_rankings: &Vec<usize>,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
        suffix_array: &mut Vec<usize>,
    ) {
        let mut position = 0;

        if cfg!(feature = "verbose") {
            println!(
                "{}> CPP node: {:?}",
                "=".repeat(self_node.suffix_len),
                self_rankings,
            );
        }

        for (_, child_node) in &self_node.children {
            // CHILD NODE: Window for Comparing Rankings using "RULES"
            let (
                //
                min_father,
                max_father,
                child_new_rankings,
            ) = self.in_prefix_merge_get_min_max_and_new_rankings(
                child_node.suffix_len,
                &child_node.rankings,
                self_rankings, // As Parent Node's Rankings.
                position,
                ip_merge_params,
                monitor,
            );

            // SELF COMMON PREFIX PARTITION: Self Node's Rankings from left to Min Father
            if position < min_father {
                // There are some Self Node's Rankings from "position" to "min_father".

                let portion_to_insert = &self_rankings[position..min_father];

                if cfg!(feature = "verbose") {
                    println!(
                        "{}. SA insert: {:?}",
                        ".".repeat(self_node.suffix_len),
                        portion_to_insert,
                    );
                }

                suffix_array.extend(portion_to_insert);
                // result_cpp.extend(portion_to_insert);
                // position = min_father;
            }
            position = max_father;

            // SELF COMMON PREFIX PARTITION: Child Node's Rankings
            if let Some(child_new_rankings) = child_new_rankings {
                self.in_prefix_merge_and_get_common_prefix_partition(
                    &child_node,
                    &child_new_rankings,
                    ip_merge_params,
                    monitor,
                    suffix_array,
                );
            } else {
                self.in_prefix_merge_and_get_common_prefix_partition(
                    &child_node,
                    &child_node.rankings,
                    ip_merge_params,
                    monitor,
                    suffix_array,
                );
            };
        }

        // SELF COMMON PREFIX PARTITION: Self Node's Rankings remained
        if position < self_rankings.len() {
            let portion_to_insert = &self_rankings[position..];

            if cfg!(feature = "verbose") {
                println!(
                    "{}. SA insert: {:?}",
                    ".".repeat(self_node.suffix_len),
                    portion_to_insert,
                );
            }

            suffix_array.extend(portion_to_insert);
            // result_cpp.extend(portion_to_insert);
            // position = self_rankings.len(); // Here useless but meaningful.
        }
    }
    fn in_prefix_merge_get_min_max_and_new_rankings(
        &self,
        self_ls_size: usize,
        self_rankings: &Vec<usize>,
        parent_rankings: &Vec<usize>,
        parent_left_position: usize,
        ip_merge_params: &mut IPMergeParams,
        monitor: &mut Monitor,
    ) -> (
        usize,              // Min Father (incl.)
        usize,              // Max Father (excl.)
        Option<Vec<usize>>, // New Self Node's Rankings
    ) {
        let str = ip_merge_params.str;

        // Compare This Node's Rankings with Parent Node's Rankings.
        let self_first_ls_index = self_rankings[0]; // Take first or another one, whatever.
        let self_ls = &str[self_first_ls_index..self_first_ls_index + self_ls_size];

        // Note: Binary Search tried before, not much of an improvement :_(

        // IN-PREFIX MERGE RANKINGS
        let mut i_parent = parent_left_position;
        while i_parent < parent_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str
                [curr_parent_ls_index..usize::min(curr_parent_ls_index + self_ls_size, str.len())];
            // Safety is required here: "..curr_parent_ls_index + self_ls_size"

            // TODO: Monitor string compare
            monitor.execution_outcome.monitor_new_local_suffix_compare();

            if curr_parent_ls < self_ls {
                // Ok. All Parent Rankings from left to here have LSs that are < than Curr LS.
            } else {
                // Found a Parent LS that is >= Curr LS.
                break;
            }
            i_parent += 1;
        }
        let min_father = i_parent;

        if min_father >= parent_rankings.len() {
            // Here, all LSs in "parent_rankings" are < than Curr LS.

            let max_father = i_parent;

            if cfg!(feature = "verbose") {
                let parent_left = &parent_rankings[parent_left_position..min_father];
                let parent_window = &parent_rankings[min_father..max_father];
                let parent_right = &parent_rankings[max_father..];
                println!(
                    "{}# In-prefix merge: Parent Rankings={:?}, Self Rankings={:?} -> {:?} smaller, {:?} equal, {:?} greater",
                    " ".repeat(self_ls_size), &parent_rankings[parent_left_position..],
                    self_rankings, parent_left, parent_window, parent_right,
                );
            }

            return (min_father, max_father, None);
        }

        // From here, we have a Min Father value. So it's true that there is at least one
        // Parent Ranking that have LS that is >= than Curr LS.

        // If Curr Parent Ranking has LS that is > than Curr LS then Max Father = Min Father. So
        // there would be no Window for Comparing Rankings using "RULES", since from here on all
        // Parent Rankings would have LSs that are > than Curr LS.

        // If Curr Parent Ranking has LS that is = to Curr LS then we'll use Max Father to define
        // a Window for Comparing Rankings using "RULES".

        let curr_parent_ls_index = parent_rankings[i_parent];
        let curr_parent_ls =
            &str[curr_parent_ls_index..usize::min(curr_parent_ls_index + self_ls_size, str.len())];
        // Seems like safety is optional here: "..curr_parent_ls_index + self_ls_size"

        // TODO: Monitor string compare
        monitor.execution_outcome.monitor_new_local_suffix_compare();

        if curr_parent_ls == self_ls {
            // Go further.
        } else {
            // Here, there aren't Parent LSs that are = to Curr LS, so Max Father = Min Father.
            // There's no Window for Comparing Rankings using "RULES".

            let max_father = min_father;

            if cfg!(feature = "verbose") {
                let parent_left = &parent_rankings[parent_left_position..min_father];
                let parent_window = &parent_rankings[min_father..max_father];
                let parent_right = &parent_rankings[max_father..];
                println!(
                    "{}# In-prefix merge: Parent Rankings={:?}, Self Rankings={:?} -> {:?} smaller, {:?} equal, {:?} greater",
                    " ".repeat(self_ls_size), &parent_rankings[parent_left_position..],
                    self_rankings, parent_left, parent_window, parent_right,
                );
            }

            return (min_father, max_father, None);
        }

        // Assuming "curr_parent_ls == this_ls".

        // There is at least one Parent Ranking that is = to Curr LS. This means that there is a
        // Window for Comparing Rankings using "RULES" to create. So now we are looking for the
        // Max Father for closing this Window.
        i_parent += 1;
        while i_parent < parent_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str
                [curr_parent_ls_index..usize::min(curr_parent_ls_index + self_ls_size, str.len())];
            // Seems like safety is optional here: "..curr_parent_ls_index + self_ls_size"

            // TODO: Monitor string compare
            monitor.execution_outcome.monitor_new_local_suffix_compare();

            if curr_parent_ls == self_ls {
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
        if cfg!(feature = "verbose") {
            let parent_left = &parent_rankings[parent_left_position..min_father];
            let parent_window = &parent_rankings[min_father..max_father];
            let parent_right = &parent_rankings[max_father..];
            println!(
                "{}# In-prefix merge: Parent Rankings={:?}, Self Rankings={:?} -> {:?} smaller, {:?} equal, {:?} greater",
                " ".repeat(self_ls_size), &parent_rankings[parent_left_position..],
                self_rankings, parent_left, parent_window, parent_right,
            );
        }

        // TODO: Avoid cloning Rankings into auxiliary memory
        let mut new_self_rankings = Vec::new();
        let mut j_this = 0;
        while i_parent < max_father && j_this < self_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_this_ls_index = self_rankings[j_this];
            let result_rules = rules_safe(
                curr_parent_ls_index,
                curr_this_ls_index,
                self_ls_size,
                &ip_merge_params.str,
                &ip_merge_params.icfl_indexes,
                &ip_merge_params.idx_to_is_custom,
                &ip_merge_params.idx_to_icfl_factor,
                monitor,
                false,
            );
            if !result_rules {
                if cfg!(feature = "verbose") {
                    let curr_parent_ls =
                        &str[curr_parent_ls_index..curr_parent_ls_index + self_ls_size];
                    // Seems like safety is optional here: "..usize::min"
                    let curr_this_ls = &str[curr_this_ls_index..curr_this_ls_index + self_ls_size];
                    println!(
                        "{}/ compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                        " ".repeat(self_ls_size), curr_parent_ls, curr_parent_ls_index,
                        curr_this_ls, curr_this_ls_index, self_ls_size,
                    );
                }
                new_self_rankings.push(curr_parent_ls_index);
                i_parent += 1;
            } else {
                if cfg!(feature = "verbose") {
                    let curr_parent_ls =
                        &str[curr_parent_ls_index..curr_parent_ls_index + self_ls_size];
                    // Seems like safety is optional here: "..usize::min"
                    let curr_this_ls = &str[curr_this_ls_index..curr_this_ls_index + self_ls_size];
                    println!(
                        "{}/ compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
                        " ".repeat(self_ls_size), curr_parent_ls, curr_parent_ls_index,
                        curr_this_ls, curr_this_ls_index, self_ls_size,
                    );
                }
                new_self_rankings.push(curr_this_ls_index);
                j_this += 1;
            }
        }
        if j_this >= self_rankings.len() {
            if cfg!(feature = "verbose") {
                println!(
                    "{}/ no child rankings left to add",
                    " ".repeat(self_ls_size),
                );
            }
        }
        while j_this < self_rankings.len() {
            let curr_this_ls_index = self_rankings[j_this];
            if cfg!(feature = "verbose") {
                println!(
                    "{}/ adding   child=\"{}\" [{}], child.suff.len={}",
                    " ".repeat(self_ls_size),
                    &str[curr_this_ls_index..curr_this_ls_index + self_ls_size],
                    curr_this_ls_index,
                    self_ls_size,
                );
            }
            new_self_rankings.push(curr_this_ls_index);
            j_this += 1;
        }
        while i_parent < max_father {
            let curr_parent_ls_index = parent_rankings[i_parent];
            if cfg!(feature = "verbose") {
                println!(
                    "{}/ adding  father=\"{}\" [{}], father.suff.len={}",
                    " ".repeat(self_ls_size),
                    &str[curr_parent_ls_index..curr_parent_ls_index + self_ls_size],
                    curr_parent_ls_index,
                    self_ls_size,
                );
            }
            new_self_rankings.push(curr_parent_ls_index);
            i_parent += 1;
        }

        // From here, the *NEW* Self Node's Rankings are "new_self_rankings".
        (min_father, max_father, Some(new_self_rankings))
    }
}
