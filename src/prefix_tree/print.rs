use crate::prefix_tree::tree::{Tree, TreeNode};

impl Tree {
    pub fn print(&self, str: &[char]) {
        self.print_node(&self.root, 0, "", str);
    }
    fn print_node(&self, self_node: &TreeNode, tabs_offset: usize, self_label: &str, str: &[char]) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            self_label,
            format!("{:?}", self_node.rankings),
        );
        for (child_node_label_pq, child_node) in &self_node.children {
            let (child_node_label_p, child_node_label_q) = *child_node_label_pq;
            let child_node_label = &str[child_node_label_p..child_node_label_q];
            let child_label = get_string_clone(child_node_label);
            self.print_node(
                child_node,
                tabs_offset + 1,
                &format!("{}{}", self_label, child_label),
                str,
            );
        }
    }
}

pub fn get_string_clone(str_type: &[char]) -> String {
    // TODO: Needs cloning
    String::from_iter(str_type)
}
