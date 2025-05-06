use crate::suffix_array::prefix_tree::new_tree::{Tree, TreeNode};
use std::cell::RefMut;

impl<'a> Tree<'a> {
    pub fn prepare_get_common_prefix_partition(&self, sa: &mut Vec<usize>, verbose: bool) {
        for &(_, first_layer_node_id) in &self.get_root().borrow().children {
            // Visiting from all First Layer Nodes to all Leafs (avoiding Root Node).
            let first_layer_node = self.get_node(first_layer_node_id).borrow_mut();
            sa.extend(self.get_common_prefix_partition(&first_layer_node, verbose));
        }
    }
    fn get_common_prefix_partition(
        &self,
        self_node: &RefMut<TreeNode<'a>>,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result = Vec::new();

        let this_rankings = &self_node.rankings;
        let mut position = 0;

        for &(_, child_node_id) in &self_node.children {
            let child_node = self.get_node(child_node_id).borrow_mut();
            let child_cpp = self.get_common_prefix_partition(&child_node, verbose);

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
                    result.extend(&this_rankings[position..min_father]);
                    position = min_father;
                }
                if let Some(max_father) = child_node.max {
                    // Here, there is both a Min Father and a Max Father, this means that there
                    // exist a Window for Comparing Rankings using "RULES". This means that all
                    // Self Node's Rankings from Min Father to Max Father should be ignored since
                    // they were duplicated (during In-prefix Merge Phase) somewhere inside its
                    // Children's Rankings, so they will be dealt with later (when these Children
                    // Nodes become Self Nodes for this procedure).
                    position = max_father;
                }
            } else {
                // Min Father is None. There's no Window for Comparing Rankings using "RULES". This
                // means that:
                // all Self Node's Rankings have LSs that are < than all Child Node's Rankings.
                // So we take firstly Self Node's Rankings, and then all the Child Node's Rankings.
                result.extend(&this_rankings[position..]);
                position = this_rankings.len();
            }

            // Add all Child Node's Rankings.
            result.extend(child_cpp);
        }

        result.extend(&this_rankings[position..]);

        if verbose {
            // Unfortunately, we don't have "self_node_id" :(
            println!(
                "Node ID=? (m={:?}, M={:?}) {:?} => {:?}",
                self_node.min, self_node.max, this_rankings, result
            );
        }
        result
    }
}
