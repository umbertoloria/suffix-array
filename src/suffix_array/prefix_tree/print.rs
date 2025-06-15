use crate::suffix_array::prefix_tree::tree::{Tree, TreeNode};

impl<'a> Tree<'a> {
    pub fn print(&self) {
        self.print_node(&self.root, 0, "");
    }
    fn print_node(&self, self_node: &TreeNode<'a>, tabs_offset: usize, self_label: &str) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            self_label,
            format!("{:?}", self_node.rankings),
        );
        for (child_node_prefix, child_node) in &self_node.children {
            let child_node_prefix = *child_node_prefix;
            let prefix_str = get_string_clone(child_node_prefix);
            let child_node_label = format!("{}{}", self_label, prefix_str);
            self.print_node(child_node, tabs_offset + 1, &child_node_label);
        }
    }
}

pub fn get_string_clone(str_type: &[u8]) -> String {
    // TODO: Needs cloning
    let cloned_vec = str_type.to_vec();
    String::from_utf8(cloned_vec).unwrap()
}
