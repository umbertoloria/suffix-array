use crate::prefix_tree::rules::rules;
use crate::prefix_tree::tree::{Tree, TreeNode};

impl<'a> Tree<'a> {
    pub fn compute_suffix_array(
        &self,
        str: &str,
        icfl_indexes: &Vec<usize>,
        idx_to_is_custom: &Vec<bool>,
        idx_to_icfl_factor: &Vec<usize>,
    ) -> Vec<usize> {
        let mut suffix_array = Vec::with_capacity(str.len());
        for (_, child_node) in &self.root.children {
            // Visiting from all First Layer Nodes to all Leafs (avoiding Root Node).
            self.get_common_prefix_partition(
                child_node,
                &child_node.rankings,
                str,
                icfl_indexes,
                idx_to_is_custom,
                idx_to_icfl_factor,
                &mut suffix_array,
            );
        }
        suffix_array
    }
    fn get_common_prefix_partition(
        &self,
        self_node: &TreeNode<'a>,
        self_rks: &Vec<usize>,
        str: &str,
        icfl_indexes: &Vec<usize>,
        idx_to_is_custom: &Vec<bool>,
        idx_to_icfl_factor: &Vec<usize>,
        suffix_array: &mut Vec<usize>,
    ) {
        let mut position = 0;
        for (_, child_node) in &self_node.children {
            let (
                //
                win_min,
                win_max,
                child_new_rankings,
            ) = self.calculate_windows_and_child_shared_rankings(
                child_node.suffix_len,
                &child_node.rankings,
                self_rks, // As Parent's Rankings.
                position,
                str,
                icfl_indexes,
                idx_to_is_custom,
                idx_to_icfl_factor,
            );
            // SELF CPP: Self Rankings from left to Child WIN-MIN.
            if position < win_min {
                let portion_to_insert = &self_rks[position..win_min];
                suffix_array.extend(portion_to_insert);
                // position = win_min; // Here useless but meaningful.
            }
            position = win_max;
            // SELF CPP: Child Rankings
            if let Some(child_new_rankings) = child_new_rankings {
                self.get_common_prefix_partition(
                    &child_node,
                    &child_new_rankings,
                    str,
                    icfl_indexes,
                    idx_to_is_custom,
                    idx_to_icfl_factor,
                    suffix_array,
                );
            } else {
                self.get_common_prefix_partition(
                    &child_node,
                    &child_node.rankings,
                    str,
                    icfl_indexes,
                    idx_to_is_custom,
                    idx_to_icfl_factor,
                    suffix_array,
                );
            };
        }
        // SELF CPP: Self Rankings left
        if position < self_rks.len() {
            let portion_to_insert = &self_rks[position..];
            suffix_array.extend(portion_to_insert);
            // position = self_rks.len(); // Here useless but meaningful.
        }
    }
    fn calculate_windows_and_child_shared_rankings(
        &self,
        self_ls_size: usize,
        self_rks: &Vec<usize>,
        parent_rks: &Vec<usize>,
        parent_rks_i_from: usize,
        str: &str,
        icfl_indexes: &Vec<usize>,
        idx_to_is_custom: &Vec<bool>,
        idx_to_icfl_factor: &Vec<usize>,
    ) -> (
        usize,              // Win. Min (incl.)
        usize,              // Win. Max (excl.)
        Option<Vec<usize>>, // New Self Node's Rankings
    ) {
        let self_ls = &str[self_rks[0]..self_rks[0] + self_ls_size];
        let mut i_parent = parent_rks_i_from;
        while i_parent < parent_rks.len() {
            let curr_parent_ls_index = parent_rks[i_parent];
            let curr_parent_ls = &str
                [curr_parent_ls_index..usize::min(curr_parent_ls_index + self_ls_size, str.len())];
            if curr_parent_ls >= self_ls {
                // Found a Parent LS that is >= Self LS.
                break;
            }
            // Until now, Parent LSs are < Self LS.
            i_parent += 1;
        }
        let win_min = i_parent;
        if win_min >= parent_rks.len() {
            // All Parent LSs are < Self LS.
            let win_max = i_parent;
            return (win_min, win_max, None);
        }
        // Curr. Parent LS is the first >= Self LS.
        let curr_parent_ls_index = parent_rks[i_parent];
        let curr_parent_ls =
            &str[curr_parent_ls_index..usize::min(curr_parent_ls_index + self_ls_size, str.len())];
        if curr_parent_ls > self_ls {
            // Curr. Parent LS is the first > Self LS.
            // There is no Parent LS = Self LS, so min=max.
            let win_max = win_min;
            return (win_min, win_max, None);
        }
        // Curr. Parent LS is the first = Self LS.
        i_parent += 1;
        while i_parent < parent_rks.len() {
            let curr_parent_ls_index = parent_rks[i_parent];
            let curr_parent_ls = &str
                [curr_parent_ls_index..usize::min(curr_parent_ls_index + self_ls_size, str.len())];
            if curr_parent_ls > self_ls {
                // Found a Parent LS that is > Self LS.
                break;
            }
            // Until now, Parent LSs are <= Self LS (before < now =).
            i_parent += 1;
        }
        let win_max = i_parent;
        i_parent = win_min;
        // The Window for Comparing Rankings using "RULES":
        // * starts from "i_parent" (included);
        // * ends with "win_max" (excluded).
        let mut new_self_rks = Vec::new();
        let mut j_self = 0;
        while i_parent < win_max && j_self < self_rks.len() {
            let curr_parent_ls_index = parent_rks[i_parent];
            let curr_self_ls_index = self_rks[j_self];
            let result_rules = rules(
                curr_parent_ls_index,
                curr_self_ls_index,
                self_ls_size,
                str,
                icfl_indexes,
                idx_to_is_custom,
                idx_to_icfl_factor,
            );
            if !result_rules {
                new_self_rks.push(curr_parent_ls_index);
                i_parent += 1;
            } else {
                new_self_rks.push(curr_self_ls_index);
                j_self += 1;
            }
        }
        while j_self < self_rks.len() {
            let curr_self_ls_index = self_rks[j_self];
            new_self_rks.push(curr_self_ls_index);
            j_self += 1;
        }
        while i_parent < win_max {
            let curr_parent_ls_index = parent_rks[i_parent];
            new_self_rks.push(curr_parent_ls_index);
            i_parent += 1;
        }
        (win_min, win_max, Some(new_self_rks))
    }
}
