use crate::suffix_array::prefix_tree::prefix_tree::{PrefixTree, PrefixTreeNode};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

impl PrefixTree {
    pub fn print(&self, str: &str, prog_sa: &ProgSuffixArray) {
        println!("PrefixTree:");
        for child in &self.children {
            child.print(str, prog_sa, 1);
        }
    }
}

impl PrefixTreeNode {
    pub fn print(&self, str: &str, prog_sa: &ProgSuffixArray, tabs_offset: usize) {
        let rankings = prog_sa.get_rankings(self.index);
        println!(
            "{}\"{}\" {:?}   m={} M={}",
            "\t".repeat(tabs_offset),
            self.get_label_from_first_ranking(str, rankings),
            rankings,
            if let Some(x) = self.min_father {
                format!("{}", x)
            } else {
                "-1".into()
            },
            if let Some(x) = self.max_father {
                format!("{}", x)
            } else {
                "-1".into()
            },
        );
        for child in &self.children {
            child.print(str, prog_sa, tabs_offset + 1);
        }
    }
}
