use crate::suffix_array::prefix_tree::new_tree::Tree;
use crate::suffix_array::prefix_trie::tree_bank_min_max::TreeBankMinMax;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl<'a> Tree<'a> {
    pub fn prepare_get_common_prefix_partition(
        &self,
        sa: &mut Vec<usize>,
        str: &str,
        prog_sa: &ProgSuffixArray,
        tree_bank_min_max: &TreeBankMinMax,
        verbose: bool,
    ) {
        for &(_, child_node_id) in &self.get_root().borrow().children {
            sa.extend(self.get_common_prefix_partition(
                child_node_id,
                str,
                prog_sa,
                tree_bank_min_max,
                verbose,
            ));
        }
    }
    fn get_common_prefix_partition(
        &self,
        self_node_id: usize,
        str: &str,
        prog_sa: &ProgSuffixArray,
        tree_bank_min_max: &TreeBankMinMax,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result = Vec::new();

        let this_rankings = prog_sa.get_rankings(self_node_id);
        let mut position = 0;
        for &(_, child_node_id) in &self.get_node(self_node_id).borrow().children {
            let child_cpp = self.get_common_prefix_partition(
                child_node_id,
                str,
                prog_sa,
                tree_bank_min_max,
                verbose,
            );
            let child_node_min_max = tree_bank_min_max.get(child_node_id);
            if let Some(min_father) = child_node_min_max.min_father {
                if verbose {
                    println!("Here self=?? and child=??");
                }
                if min_father >= position {
                    result.extend(&this_rankings[position..min_father]);
                }
                result.extend(child_cpp);
                if let Some(max_father) = child_node_min_max.max_father {
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
            let self_node_min_max = tree_bank_min_max.get(self_node_id);
            println!(
                "Node ID={} (m={:?}, M={:?}) {:?} => {:?}",
                self_node_id,
                self_node_min_max.min_father,
                self_node_min_max.max_father,
                this_rankings,
                result
            );
        }
        result
    }
}
