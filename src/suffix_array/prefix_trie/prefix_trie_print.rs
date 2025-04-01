use crate::suffix_array::prefix_trie::prefix_trie::{
    get_string_char_clone, get_string_clone, PrefixTrie, PrefixTrieData,
};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl<'a> PrefixTrie<'a> {
    pub fn get_label_from_first_ranking(&self, str: &'a str, rankings: &[usize]) -> &'a str {
        // Make sure this node is not the Root Node, because it's the only one that has no Rankings.
        let first_ranking = rankings[0];
        &str[first_ranking..first_ranking + self.suffix_len]
    }
    pub fn print(&self, tabs_offset: usize, self_label: &str, str: &str) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            self_label,
            format!(
                "{:?} {:?}",
                self.get_rankings_canonical(),
                self.get_rankings_custom()
            ),
        );
        match &self.data {
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    let prefix_str = get_string_char_clone(*char_key);
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print(tabs_offset + 1, &child_node_label, str);
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                let child_node_label = format!("{}{}", self_label, prefix_str);
                child_node.print(tabs_offset + 1, &child_node_label, str);
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let mut prefix_str = "";
                    if !child_node.rankings_canonical.is_empty() {
                        prefix_str = child_node
                            .get_label_from_first_ranking(str, &child_node.rankings_canonical);
                    } else if !child_node.rankings_custom.is_empty() {
                        prefix_str = child_node
                            .get_label_from_first_ranking(str, &child_node.rankings_custom);
                    }
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print(tabs_offset + 1, &child_node_label, str);
                }
            }
        }
    }
    pub fn print_merged(
        &self,
        tabs_offset: usize,
        self_label: &str,
        str: &str,
        prog_sa: &ProgSuffixArray,
    ) {
        let self_rankings = prog_sa.get_rankings(self.id);
        println!(
            "{}\"{}\" {:?}",
            "\t".repeat(tabs_offset),
            self_label,
            self_rankings,
        );
        match &self.data {
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    let prefix_str = get_string_char_clone(*char_key);
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print_merged(tabs_offset + 1, &child_node_label, str, prog_sa);
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                let child_node_label = format!("{}{}", self_label, prefix_str);
                child_node.print_merged(tabs_offset + 1, &child_node_label, str, prog_sa);
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let child_rankings = prog_sa.get_rankings(child_node.id);
                    let prefix_str = child_node.get_label_from_first_ranking(str, child_rankings);
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print_merged(tabs_offset + 1, &child_node_label, str, prog_sa);
                }
            }
        }
    }
}
