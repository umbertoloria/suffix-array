use crate::suffix_array::prefix_tree::prefix_tree::{PrefixTree, PrefixTreeNode};
use crate::suffix_array::prefix_trie::node_father_bank::NodeFatherBank;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl PrefixTree {
    pub fn prepare_get_common_prefix_partition(
        &self,
        sa: &mut Vec<usize>,
        str: &str,
        prog_sa: &ProgSuffixArray,
        node_father_bank: &NodeFatherBank,
        verbose: bool,
    ) {
        for child_node in &self.children {
            sa.extend(child_node.get_common_prefix_partition(
                str,
                prog_sa,
                node_father_bank,
                verbose,
            ));
        }
    }
}
impl PrefixTreeNode {
    pub fn get_label_from_first_ranking<'a>(&self, str: &'a str, rankings: &[usize]) -> &'a str {
        // Make sure this node is not the Root Node, because it's the only one that has no Rankings.
        let first_ranking = rankings[0];
        &str[first_ranking..first_ranking + self.suffix_len]
    }
    fn get_common_prefix_partition(
        &self,
        str: &str,
        prog_sa: &ProgSuffixArray,
        node_father_bank: &NodeFatherBank,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result = Vec::new();

        let this_rankings = prog_sa.get_rankings(self.index);
        let mut position = 0;
        for child_node in &self.children {
            let child_cpp = child_node.get_common_prefix_partition(
                //
                str,
                prog_sa,
                node_father_bank,
                verbose,
            );
            let child_node_data = node_father_bank.get_node_data(child_node.index);
            if let Some(min_father) = child_node_data.min_father {
                if verbose {
                    println!("Here self=?? and child=??");
                }
                if min_father >= position {
                    result.extend(&this_rankings[position..min_father]);
                }
                result.extend(child_cpp);
                if let Some(max_father) = child_node_data.max_father {
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
            let self_node_data = node_father_bank.get_node_data(self.index);
            println!(
                "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                self.get_label_from_first_ranking(str, this_rankings),
                self_node_data.min_father,
                self_node_data.max_father,
                this_rankings,
                result
            );
        }
        result
    }
}
