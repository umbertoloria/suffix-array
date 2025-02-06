use crate::suffix_array::prefix_trie::PrefixTrie;
use std::fs::{create_dir_all, File};
use std::io::Write;

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
    pub fn prepare_get_common_prefix_partition(&mut self, sa: &mut Vec<usize>, verbose: bool) {
        for first_layer_son in &mut self.children {
            sa.extend(first_layer_son.get_common_prefix_partition(verbose));
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
    fn get_common_prefix_partition(&mut self, verbose: bool) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        /*println!("\nNode: ");
        println!("{}", self.label);*/

        let common = &self.rankings;
        /*println!("common: ");
        println!("{:?}", common);*/

        if self.children.is_empty() {
            result.extend(common);
            if verbose {
                println!(
                    "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                    self.label, self.min_father, self.max_father, self.rankings, result
                );
            }
            /*println!("result: ");
            println!("{:?}", result);*/
            return result;
        }

        let mut position = 0;
        for son in &mut self.children {
            let temp = son.get_common_prefix_partition(verbose);
            if let Some(min_father) = son.min_father {
                if verbose {
                    println!("Here self={} and son={}", self.label, son.label);
                }
                if min_father >= position {
                    result.extend(&common[position..min_father]);
                }
                result.extend(temp);
                if let Some(max_father) = son.max_father {
                    position = max_father;
                } else {
                    position = min_father;
                }
            } else {
                // Min Father is None.
                result.extend(&common[position..]);
                result.extend(temp);
                position = common.len();
            }
        }
        result.extend(&common[position..]);

        if verbose {
            println!(
                "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                self.label, self.min_father, self.max_father, self.rankings, result
            );
        }
        /*println!("result: ");
        println!("{:?}", result);*/

        result
    }
}
pub fn create_prefix_tree_from_prefix_trie(root_trie: PrefixTrie, wbsa: &Vec<usize>) -> PrefixTree {
    let mut tree = PrefixTree {
        children: create_prefix_tree_from_trie_deep(&root_trie, wbsa),
    };
    tree
}
fn create_prefix_tree_from_trie_deep(
    real_node: &PrefixTrie,
    wbsa: &Vec<usize>,
) -> Vec<PrefixTreeNode> {
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
        for son in real_node.sons.values() {
            let nodes_list = create_prefix_tree_from_trie_deep(son, wbsa);
            node.children.extend(nodes_list);
        }
        result.push(node);
    } else {
        // This Node is a Bridge, so we consider its Children (skipping Child Bridges).
        for child in real_node.sons.values() {
            let nodes_list = create_prefix_tree_from_trie_deep(child, wbsa);
            result.extend(nodes_list);
        }
    }

    result
}

// PREFIX TREE LOGGER
pub fn log_prefix_tree(prefix_tree: &PrefixTree, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for child in &prefix_tree.children {
        log_prefix_tree_recursive(child, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_tree_recursive(node: &PrefixTreeNode, file: &mut File, level: usize) {
    let mut line = format!("{}{}", " ".repeat(level), node.label);
    let rankings = &node.rankings;
    if !rankings.is_empty() {
        line.push_str(" [");
        let last_ranking = rankings[rankings.len() - 1];
        for i in 0..rankings.len() - 1 {
            let ranking = rankings[i];
            line.push_str(&format!("{}, ", ranking));
        }
        line.push_str(&format!("{}]", last_ranking));
    }
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for child in &node.children {
        log_prefix_tree_recursive(child, file, level + 1);
    }
}

// SUFFIX ARRAY LOGGER
pub fn make_sure_directory_exist(folder_path: String) {
    create_dir_all(folder_path).unwrap();
}
pub fn log_suffix_array(sa: &Vec<usize>, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for sa_item in sa {
        file.write(format!("{}\n", sa_item).as_bytes())
            .expect("Unable to write");
    }
    file.flush().expect("Unable to flush file");
}
