use crate::suffix_array::prefix_tree::prefix_tree::{PrefixTree, PrefixTreeNode};
use crate::suffix_array::prefix_trie::prefix_trie::{PrefixTrie, PrefixTrieData};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

pub fn create_prefix_tree_from_prefix_trie(
    root_trie: PrefixTrie,
    prog_sa_trie: &ProgSuffixArray,
    prog_sa: &mut ProgSuffixArray,
) -> PrefixTree {
    let mut next_node_index = 0;

    // Prefix Tree Node
    let mut node = PrefixTree {
        children: Vec::new(),
    };

    // Add children
    match root_trie.data {
        PrefixTrieData::Children(children) => {
            for (_, child_node) in children {
                let (child_tree_node, next_node_index_) = create_prefix_tree_from_trie_deep(
                    child_node,
                    prog_sa_trie,
                    prog_sa,
                    next_node_index,
                );
                node.children.push(child_tree_node);
                next_node_index = next_node_index_;
            }
        }
        PrefixTrieData::DirectChild((_, child_node)) => {
            let (child_tree_node, next_node_index_) = create_prefix_tree_from_trie_deep(
                *child_node,
                prog_sa_trie,
                prog_sa,
                next_node_index,
            );
            node.children.push(child_tree_node);
            next_node_index = next_node_index_;
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
        PrefixTrieData::Vec(children) => {
            for child_node in children {
                let (child_tree_node, next_node_index_) = create_prefix_tree_from_trie_deep(
                    child_node,
                    prog_sa_trie,
                    prog_sa,
                    next_node_index,
                );
                node.children.push(child_tree_node);
                next_node_index = next_node_index_;
            }
        }
    }
    node
}
fn create_prefix_tree_from_trie_deep(
    real_node: PrefixTrie,
    prog_sa_trie: &ProgSuffixArray,
    prog_sa: &mut ProgSuffixArray,
    next_node_index: usize,
) -> (PrefixTreeNode, usize) {
    // Every Node here has Rankings, so we consider it.

    let mut next_node_index = next_node_index;

    // Prefix Tree Node
    let real_node_rankings = prog_sa_trie.get_rankings(real_node.id);
    prog_sa.assign_rankings_to_node_index(next_node_index, &real_node_rankings);
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
                let (child_tree_node, next_node_index_) = create_prefix_tree_from_trie_deep(
                    child_node,
                    prog_sa_trie,
                    prog_sa,
                    next_node_index,
                );
                node.children.push(child_tree_node);
                next_node_index = next_node_index_;
            }
        }
        PrefixTrieData::DirectChild((_, child_node)) => {
            let (child_tree_node, next_node_index_) = create_prefix_tree_from_trie_deep(
                *child_node,
                prog_sa_trie,
                prog_sa,
                next_node_index,
            );
            node.children.push(child_tree_node);
            next_node_index = next_node_index_;
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
        PrefixTrieData::Vec(children) => {
            for child_node in children {
                let (child_tree_node, next_node_index_) = create_prefix_tree_from_trie_deep(
                    child_node,
                    prog_sa_trie,
                    prog_sa,
                    next_node_index,
                );
                node.children.push(child_tree_node);
                next_node_index = next_node_index_;
            }
        }
    }
    (node, next_node_index)
}
