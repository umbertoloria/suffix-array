use crate::suffix_array::prefix_tree::prefix_tree::{PrefixTree, PrefixTreeNode};
use crate::suffix_array::prefix_trie::prefix_trie::{PrefixTrie, PrefixTrieData};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

pub fn create_prefix_tree_from_prefix_trie(
    root_trie: PrefixTrie,
    prog_sa_trie: &ProgSuffixArray,
    prog_sa: &mut ProgSuffixArray,
) -> PrefixTree {
    PrefixTree {
        children: create_children_tree_nodes_from_node_trie_children(
            root_trie,
            prog_sa_trie,
            prog_sa,
        ),
    }
}

fn create_children_tree_nodes_from_node_trie_children(
    node_trie: PrefixTrie,
    prog_sa_trie: &ProgSuffixArray,
    prog_sa: &mut ProgSuffixArray,
) -> Vec<PrefixTreeNode> {
    let mut node_children = Vec::new();
    match node_trie.data {
        PrefixTrieData::Children(children) => {
            for (_, child_node) in children {
                node_children.push(
                    //
                    create_prefix_tree_node_and_assign_rankings_and_index(
                        child_node,
                        prog_sa_trie,
                        prog_sa,
                    ),
                );
            }
        }
        PrefixTrieData::DirectChild((_, child_node)) => {
            node_children.push(
                //
                create_prefix_tree_node_and_assign_rankings_and_index(
                    *child_node,
                    prog_sa_trie,
                    prog_sa,
                ),
            );
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
        PrefixTrieData::Vec(children) => {
            for child_node in children {
                node_children.push(
                    //
                    create_prefix_tree_node_and_assign_rankings_and_index(
                        child_node,
                        prog_sa_trie,
                        prog_sa,
                    ),
                );
            }
        }
    }
    node_children
}

fn create_prefix_tree_node_and_assign_rankings_and_index(
    node_trie: PrefixTrie,
    prog_sa_trie: &ProgSuffixArray,
    prog_sa: &mut ProgSuffixArray,
) -> PrefixTreeNode {
    // Every Node here has Rankings, so we consider it.
    let node_index = node_trie.id;
    prog_sa.assign_rankings_to_node_index(node_index, prog_sa_trie.get_rankings(node_trie.id));
    PrefixTreeNode {
        index: node_index,
        suffix_len: node_trie.suffix_len,
        children: create_children_tree_nodes_from_node_trie_children(
            node_trie,
            prog_sa_trie,
            prog_sa,
        ),
        min_father: None,
        max_father: None,
    }
}
