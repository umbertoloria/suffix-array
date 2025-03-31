use crate::suffix_array::prefix_tree::prefix_tree::{PrefixTree, PrefixTreeNode};
use crate::suffix_array::prefix_trie::prefix_trie::{PrefixTrie, PrefixTrieData};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

pub fn create_prefix_tree_from_prefix_trie(
    root_trie: PrefixTrie,
    prog_sa: &mut ProgSuffixArray,
) -> PrefixTree {
    let (nodes_list, _) = create_prefix_tree_from_trie_deep(root_trie, prog_sa, 0);
    PrefixTree {
        children: nodes_list,
    }
}
fn create_prefix_tree_from_trie_deep(
    real_node: PrefixTrie,
    prog_sa: &mut ProgSuffixArray,
    next_node_index: usize,
) -> (Vec<PrefixTreeNode>, usize) {
    let mut result = Vec::new();
    let mut next_node_index = next_node_index;

    if real_node.rankings.len() > 0 {
        // This Node has Rankings, so we consider it.

        // Create Prefix Tree Node
        prog_sa.assign_rankings_to_node_index(next_node_index, &real_node.rankings);
        let mut node = PrefixTreeNode {
            index: next_node_index,
            suffix_len: real_node.suffix_len,
            children: Vec::new(),
            min_father: None,
            max_father: None,
        };
        next_node_index += 1;

        // Add children
        match real_node.data {
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    let (nodes_list, next_node_index_) =
                        create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                    node.children.extend(nodes_list);
                    next_node_index = next_node_index_;
                }
            }
            PrefixTrieData::DirectChild((_, child_node)) => {
                let (nodes_list, next_node_index_) =
                    create_prefix_tree_from_trie_deep(*child_node, prog_sa, next_node_index);
                node.children.extend(nodes_list);
                next_node_index = next_node_index_;
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let (nodes_list, next_node_index_) =
                        create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                    node.children.extend(nodes_list);
                    next_node_index = next_node_index_;
                }
            }
        }
        result.push(node);
    } else {
        // This Node is a Bridge, so we consider its Children (skipping Child Bridges).
        match real_node.data {
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    let (nodes_list, next_node_index_) =
                        create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                    result.extend(nodes_list);
                    next_node_index = next_node_index_;
                }
            }
            PrefixTrieData::DirectChild((_, child_node)) => {
                let (nodes_list, next_node_index_) =
                    create_prefix_tree_from_trie_deep(*child_node, prog_sa, next_node_index);
                result.extend(nodes_list);
                next_node_index = next_node_index_;
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let (nodes_list, next_node_index_) =
                        create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                    result.extend(nodes_list);
                    next_node_index = next_node_index_;
                }
            }
        }
    }

    (result, next_node_index)
}
