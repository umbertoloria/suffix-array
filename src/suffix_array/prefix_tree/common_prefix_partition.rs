use crate::suffix_array::prefix_tree::new_tree::Tree;

impl<'a> Tree<'a> {
    pub fn prepare_get_common_prefix_partition(
        &self,
        sa: &mut Vec<usize>,
        str: &str,
        verbose: bool,
    ) {
        for &(_, child_node_id) in &self.get_root().borrow().children {
            sa.extend(self.get_common_prefix_partition(child_node_id, str, verbose));
        }
    }
    fn get_common_prefix_partition(
        &self,
        self_node_id: usize,
        str: &str,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result = Vec::new();

        let self_node = self.get_node(self_node_id).borrow();
        let this_rankings = &self_node.rankings;
        let mut position = 0;
        for &(_, child_node_id) in &self_node.children {
            let child_cpp = self.get_common_prefix_partition(child_node_id, str, verbose);
            let child_node = self.get_node(child_node_id).borrow();
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
            println!(
                "Node ID={} (m={:?}, M={:?}) {:?} => {:?}",
                self_node_id, self_node.min, self_node.max, this_rankings, result
            );
        }
        result
    }
}
