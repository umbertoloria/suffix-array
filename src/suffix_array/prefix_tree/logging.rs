use crate::suffix_array::prefix_tree::print::get_string_clone;
use crate::suffix_array::prefix_tree::tree::{Tree, TreeNode};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy)]
pub enum TreeLogMode {
    Tree,
    FullTree,
    MiniTree,
}
pub fn log_tree(tree: &Tree, mode: TreeLogMode, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    // Logging from all First Layer Nodes to all Leafs (avoiding Root Node).
    for (child_node_prefix, child_node) in &tree.root.children {
        let child_node_prefix = *child_node_prefix;
        let child_label = match mode {
            TreeLogMode::Tree => format!("{}", get_string_clone(child_node_prefix)),
            TreeLogMode::FullTree => format!("{}", get_string_clone(child_node_prefix)),
            TreeLogMode::MiniTree => format!("\"{:6}\"", child_node_prefix.len()),
        };
        log_tree_recursive(&child_node, &child_label, mode, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_tree_recursive(
    node: &TreeNode,
    node_label: &str,
    mode: TreeLogMode,
    file: &mut File,
    level: usize,
) {
    let mut line = format!(
        //
        "{}{} <{}>",
        " ".repeat(level),
        node_label,
        // node_id, // Avoid showing Node ID.
        "",
    );
    let rankings = &node.rankings;
    line.push_str(" [");
    for i in 0..rankings.len() - 1 {
        let ranking = rankings[i];
        line.push_str(&format!("{}, ", ranking));
    }
    line.push_str(&format!("{}]", rankings[rankings.len() - 1]));
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for (child_node_prefix, child_node) in &node.children {
        let child_node_prefix = *child_node_prefix;
        let child_label = match mode {
            TreeLogMode::Tree => format!("{}", get_string_clone(child_node_prefix)),
            TreeLogMode::FullTree => {
                format!("{}{}", node_label, get_string_clone(child_node_prefix))
            }
            TreeLogMode::MiniTree => format!("\"{:6}\"", child_node_prefix.len()),
        };
        log_tree_recursive(child_node, &child_label, mode, file, level + 1);
    }
}
