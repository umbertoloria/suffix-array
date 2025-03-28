use std::fs::File;
use std::io::Write;
use crate::suffix_array::prefix_tree::prefix_tree::{PrefixTree, PrefixTreeNode};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

pub fn log_prefix_tree(
    prefix_tree: &PrefixTree,
    str: &str,
    prog_sa: &ProgSuffixArray,
    filepath: String,
) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for child in &prefix_tree.children {
        log_prefix_tree_recursive(child, str, prog_sa, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_tree_recursive(
    node: &PrefixTreeNode,
    str: &str,
    prog_sa: &ProgSuffixArray,
    file: &mut File,
    level: usize,
) {
    let rankings = prog_sa.get_rankings(node.index);
    let mut line = format!(
        "{}{}",
        " ".repeat(level),
        node.get_label_from_first_ranking(str, rankings)
    );
    line.push_str(" [");
    let last_ranking = rankings[rankings.len() - 1];
    for i in 0..rankings.len() - 1 {
        let ranking = rankings[i];
        line.push_str(&format!("{}, ", ranking));
    }
    line.push_str(&format!("{}]", last_ranking));
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for child in &node.children {
        log_prefix_tree_recursive(child, str, prog_sa, file, level + 1);
    }
}
