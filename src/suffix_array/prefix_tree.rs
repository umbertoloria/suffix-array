use crate::suffix_array::prefix_trie::PrefixTrie;

pub struct PrefixTree {
    pub children: Vec<PrefixTreeNode>,
}
impl PrefixTree {
    pub fn print(&self) {
        println!("PrefixTree:");
        for child in &self.children {
            child.print(1);
        }
    }
}
pub struct PrefixTreeNode {
    pub label: String,
    pub suffix_len: usize,
    pub children: Vec<PrefixTreeNode>,
    pub rankings: Vec<usize>,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTreeNode {
    pub fn print(&self, tabs_offset: usize) {
        println!(
            "{}\"{}\" {:?}   m={} M={}",
            "\t".repeat(tabs_offset),
            self.label,
            self.rankings,
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
            child.print(tabs_offset + 1);
        }
    }
}
pub fn create_pt_from_trie(root_trie: PrefixTrie, wbsa: &Vec<usize>) -> PrefixTree {
    let mut tree = PrefixTree {
        children: create_pt_from_trie_deep(&root_trie, wbsa),
    };
    tree
}
fn create_pt_from_trie_deep(real_node: &PrefixTrie, wbsa: &Vec<usize>) -> Vec<PrefixTreeNode> {
    let mut result = Vec::new();

    let rankings = real_node.get_real_rankings(wbsa);
    if rankings.len() > 0 {
        // This Node has Rankings, so we consider it.
        let mut node = PrefixTreeNode {
            label: real_node.label.clone(), // TODO: Avoid cloning
            suffix_len: real_node.suffix_len,
            children: Vec::new(),
            rankings,
            min_father: real_node.min_father,
            max_father: real_node.max_father,
        };
        for child in real_node.sons.values() {
            let nodes_list = create_pt_from_trie_deep(child, wbsa);
            node.children.extend(nodes_list);
        }
        result.push(node);
    } else {
        // This Node is a Bridge, so we consider its Children (skipping Child Bridges).
        for child in real_node.sons.values() {
            let nodes_list = create_pt_from_trie_deep(child, wbsa);
            result.extend(nodes_list);
        }
    }

    result
}
