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
            if let Some(min_father) = child_node.min {
                if verbose {
                    println!("Here self=?? and child=??");
                }
                if min_father >= position {
                    result.extend(&this_rankings[position..min_father]);
                }
                result.extend(child_cpp);
                if let Some(max_father) = child_node.max {
                    position = max_father;
                } else {
                    position = min_father;
                }
            } else {
                // Min Father is None.
                result.extend(&this_rankings[position..]);
                result.extend(child_cpp);
                position = this_rankings.len();
            }
        }

        result.extend(&this_rankings[position..]);

        if verbose {
            // Here we don't have "self_node_id" :(
            println!(
                "Node ID=? (m={:?}, M={:?}) {:?} => {:?}",
                self_node.min, self_node.max, this_rankings, result
            );
        }
        result
    }
}
