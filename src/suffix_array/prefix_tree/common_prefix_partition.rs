use crate::suffix_array::prefix_trie::prefix_trie::{PrefixTrie, PrefixTrieData};
use crate::suffix_array::prefix_trie::tree_bank_min_max::TreeBankMinMax;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl<'a> PrefixTrie<'a> {
    pub fn prepare_get_common_prefix_partition(
        &self,
        sa: &mut Vec<usize>,
        str: &str,
        prog_sa: &ProgSuffixArray,
        tree_bank_min_max: &TreeBankMinMax,
        verbose: bool,
    ) {
        match &self.data {
            PrefixTrieData::Leaf => {}
            PrefixTrieData::DirectChild((_, child_node)) => {
                sa.extend(child_node.get_common_prefix_partition(
                    str,
                    prog_sa,
                    tree_bank_min_max,
                    verbose,
                ));
            }
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    sa.extend(child_node.get_common_prefix_partition(
                        str,
                        prog_sa,
                        tree_bank_min_max,
                        verbose,
                    ));
                }
            }
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    sa.extend(child_node.get_common_prefix_partition(
                        str,
                        prog_sa,
                        tree_bank_min_max,
                        verbose,
                    ));
                }
            }
        }
    }
    pub fn get_label_from_first_ranking<'b>(&self, str: &'b str, rankings: &[usize]) -> &'b str {
        // Make sure this node is not the Root Node, because it's the only one that has no Rankings.
        let first_ranking = rankings[0];
        &str[first_ranking..first_ranking + self.suffix_len]
    }
    fn get_common_prefix_partition(
        &self,
        str: &str,
        prog_sa: &ProgSuffixArray,
        tree_bank_min_max: &TreeBankMinMax,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result = Vec::new();

        let this_rankings = prog_sa.get_rankings(self.id);
        let mut position = 0;
        match &self.data {
            PrefixTrieData::Leaf => {}
            PrefixTrieData::DirectChild((_, child_node)) => {
                let child_cpp = child_node.get_common_prefix_partition(
                    str,
                    prog_sa,
                    tree_bank_min_max,
                    verbose,
                );
                let child_node_data = tree_bank_min_max.get_min_max(child_node.id);
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
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    let child_cpp = child_node.get_common_prefix_partition(
                        str,
                        prog_sa,
                        tree_bank_min_max,
                        verbose,
                    );
                    let child_node_data = tree_bank_min_max.get_min_max(child_node.id);
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
            }
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let child_cpp = child_node.get_common_prefix_partition(
                        str,
                        prog_sa,
                        tree_bank_min_max,
                        verbose,
                    );
                    let child_node_data = tree_bank_min_max.get_min_max(child_node.id);
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
            }
        }
        result.extend(&this_rankings[position..]);

        if verbose {
            let self_node_data = tree_bank_min_max.get_min_max(self.id);
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
