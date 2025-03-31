use crate::suffix_array::prefix_trie::prefix_trie::{
    get_string_char_clone, get_string_clone, PrefixTrie, PrefixTrieData,
};
use std::fs::File;
use std::io::Write;

pub fn log_prefix_trie(root: &PrefixTrie, filepath: String, str: &str) {
    let mut file = File::create(filepath).expect("Unable to create file");
    match &root.data {
        PrefixTrieData::Children(children) => {
            for (char_key, child_node) in children {
                let child_label = get_string_char_clone(*char_key);
                log_prefix_trie_recursive(child_node, &child_label, &mut file, 0, str);
            }
        }
        PrefixTrieData::DirectChild((prefix, child_node)) => {
            let child_label = get_string_clone(prefix);
            log_prefix_trie_recursive(child_node, &child_label, &mut file, 0, str);
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
        PrefixTrieData::Vec(children) => {
            for child_node in children {
                let child_label = child_node.get_label_from_first_ranking(str);
                log_prefix_trie_recursive(child_node, &child_label, &mut file, 0, str);
            }
        }
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_trie_recursive(
    node: &PrefixTrie,
    node_label: &str,
    file: &mut File,
    level: usize,
    str: &str,
) {
    let mut line = format!("{}{}", " ".repeat(level), node_label);
    let mut rankings = &node.rankings;
    if !rankings.is_empty() {
        line.push_str(" [");
        for i in 0..rankings.len() - 1 {
            let ranking = rankings[i];
            line.push_str(&format!("{}, ", ranking));
        }
        line.push_str(&format!("{}]", rankings[rankings.len() - 1]));
    }
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    match &node.data {
        PrefixTrieData::Children(children) => {
            for (char_key, child_node) in children {
                let child_label = format!("{}{}", node_label, get_string_char_clone(*char_key));
                log_prefix_trie_recursive(child_node, &child_label, file, level + 1, str);
            }
        }
        PrefixTrieData::DirectChild((prefix, child_node)) => {
            let child_label = format!("{}{}", node_label, get_string_clone(prefix));
            log_prefix_trie_recursive(child_node, &child_label, file, level + prefix.len(), str);
        }
        PrefixTrieData::Leaf => {}
        PrefixTrieData::InitRoot => {}
        PrefixTrieData::Vec(children) => {
            for child_node in children {
                let child_label = child_node.get_label_from_first_ranking(str);
                log_prefix_trie_recursive(child_node, &child_label, file, level + 1, str);
            }
        }
    }
}
