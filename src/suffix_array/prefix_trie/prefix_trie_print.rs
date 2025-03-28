use crate::suffix_array::prefix_trie::prefix_trie::{
    get_string_char_clone, get_string_clone, PrefixTrie, PrefixTrieData,
};

impl<'a> PrefixTrie<'a> {
    pub fn print(&self, tabs_offset: usize, prefix_rec: String) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            prefix_rec,
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
                    child_node.print(tabs_offset + 1, format!("{}{}", prefix_rec, prefix_str));
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                child_node.print(tabs_offset + 1, format!("{}{}", prefix_rec, prefix_str));
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }
    }
    pub fn print_merged(&self, tabs_offset: usize, prefix_rec: String) {
        println!(
            "{}\"{}\" {:?}",
            "\t".repeat(tabs_offset),
            prefix_rec,
            self.rankings,
        );
        match &self.data {
            PrefixTrieData::Children(children) => {
                for (char_key, child_node) in children {
                    let prefix_str = get_string_char_clone(*char_key);
                    let prefix_rec = format!("{}{}", prefix_rec, prefix_str);
                    child_node.print_merged(tabs_offset + 1, prefix_rec);
                }
            }
            PrefixTrieData::DirectChild((prefix, child_node)) => {
                let prefix_str = get_string_clone(prefix);
                let prefix_rec = format!("{}{}", prefix_rec, prefix_str);
                child_node.print_merged(tabs_offset + 1, prefix_rec);
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }
    }
}
