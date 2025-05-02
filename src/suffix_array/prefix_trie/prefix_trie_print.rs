use crate::suffix_array::prefix_trie::prefix_trie::{
    get_string_char_clone, get_string_clone, PrefixTrie, PrefixTrieData,
};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl<'a> PrefixTrie<'a> {
    pub fn print_before_shrink(&self, tabs_offset: usize, self_label: &str, str: &str) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            self_label,
            format!("{:?}", self.rankings),
        );
        match &self.data {
            PrefixTrieData::Leaf => {}
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                let child_node_label = format!("{}{}", self_label, prefix_str);
                child_node.print_before_shrink(tabs_offset + 1, &child_node_label, str);
            }
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    let prefix_str = get_string_char_clone(*char_key);
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print_before_shrink(tabs_offset + 1, &child_node_label, str);
                }
            }
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let prefix_str = if !child_node.rankings.is_empty() {
                        child_node.get_label_from_first_ranking(str, &child_node.rankings)
                    } else {
                        ""
                    };
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print_before_shrink(tabs_offset + 1, &child_node_label, str);
                }
            }
        }
    }
    pub fn print_from_prog_sa(
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
            PrefixTrieData::Leaf => {}
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                let child_node_label = format!("{}{}", self_label, prefix_str);
                child_node.print_from_prog_sa(tabs_offset + 1, &child_node_label, str, prog_sa);
            }
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    let prefix_str = get_string_char_clone(*char_key);
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print_from_prog_sa(tabs_offset + 1, &child_node_label, str, prog_sa);
                }
            }
            PrefixTrieData::Vec(children) => {
                for child_node in children {
                    let child_rankings = prog_sa.get_rankings(child_node.id);
                    let prefix_str = child_node.get_label_from_first_ranking(str, child_rankings);
                    let child_node_label = format!("{}{}", self_label, prefix_str);
                    child_node.print_from_prog_sa(tabs_offset + 1, &child_node_label, str, prog_sa);
                }
            }
        }
    }
}
