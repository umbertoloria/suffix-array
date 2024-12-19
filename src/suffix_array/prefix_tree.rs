use crate::suffix_array::ls_and_rankings::LSandRankings;

pub fn is_prefix_of<'a>(needle: &'a str, haystack: &'a str) -> bool {
    // TODO: These params should be typed "String"?
    if haystack.len() < needle.len() {
        false
    } else {
        haystack[0..needle.len()].eq(needle)
    }
}

pub struct PrefixTreeNode {
    str: String,
    children: Vec<PrefixTreeNode>,
}
impl PrefixTreeNode {
    fn new(str: String) -> Self {
        Self {
            str,
            children: Vec::new(),
        }
    }

    pub fn add_string(&mut self, str: String) {
        // TODO: Avoid recursive?
        for child in &mut self.children {
            if is_prefix_of(child.str.as_str(), str.as_str()) {
                child.add_string(str);
                return;
            }
        }
        self.children.push(PrefixTreeNode::new(str));
    }

    pub fn show_tree(&self, tab: usize) {
        println!("{}└--* {}", "|  ".repeat(tab), self.str);
        for node in &self.children {
            node.show_tree(tab + 1);
        }
    }
}

pub fn create_prefix_tree_from_ls_and_rankings(ls_and_rankings: &LSandRankings) -> PrefixTreeNode {
    let mut root_node = PrefixTreeNode {
        str: String::from(""), // First string is empty (it's a "sentinel" let's say).
        children: Vec::new(),
    };
    for s_index in 0..ls_and_rankings.ls_list.len() {
        let (s, s_ranking) = ls_and_rankings.get_s_and_ranking_by_index(s_index);
        root_node.add_string(s.into());
    }
    root_node
}
