use crate::suffix_array::prefix_trie::{PrefixTrie, WbsaIndexes};
use std::fs::{create_dir_all, File};
use std::io::Write;

pub struct PrefixTree {
    pub children: Vec<PrefixTreeNode>,
}
impl PrefixTree {
    pub fn print(&self, str: &str) {
        println!("PrefixTree:");
        for child in &self.children {
            child.print(str, 1);
        }
    }
    pub fn prepare_get_common_prefix_partition(
        &mut self,
        sa: &mut Vec<usize>,
        str: &str,
        verbose: bool,
    ) {
        for first_layer_son in &mut self.children {
            sa.extend(first_layer_son.get_common_prefix_partition(str, verbose));
        }
    }
}
pub struct PrefixTreeNode {
    pub suffix_len: usize,
    pub children: Vec<PrefixTreeNode>,
    pub rankings: Vec<usize>,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTreeNode {
    fn get_label<'a>(&self, str: &'a str) -> &'a str {
        let first_ranking = self.rankings[0];
        &str[first_ranking..first_ranking + self.suffix_len]
    }
    pub fn print(&self, str: &str, tabs_offset: usize) {
        println!(
            "{}\"{}\" {:?}   m={} M={}",
            "\t".repeat(tabs_offset),
            self.get_label(str),
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
            child.print(str, tabs_offset + 1);
        }
    }
    fn get_common_prefix_partition(&mut self, str: &str, verbose: bool) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        let common = &self.rankings;

        if self.children.is_empty() {
            result.extend(common);
            if verbose {
                println!(
                    "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                    self.get_label(str),
                    self.min_father,
                    self.max_father,
                    self.rankings,
                    result
                );
            }
            return result;
        }

        let mut position = 0;
        for son in &mut self.children {
            let temp = son.get_common_prefix_partition(str, verbose);
            if let Some(min_father) = son.min_father {
                if verbose {
                    println!(
                        "Here self=?? and son=??",
                        // self.get_label(str),
                        // son.get_label(str)
                    );
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
                self.get_label(str),
                self.min_father,
                self.max_father,
                self.rankings,
                result
            );
        }

        result
    }
}
pub fn create_prefix_tree_from_prefix_trie(
    root_trie: PrefixTrie,
    wbsa: &Vec<usize>,
    wbsa_indexes: &mut WbsaIndexes,
) -> PrefixTree {
    let mut tree = PrefixTree {
        children: create_prefix_tree_from_trie_deep(root_trie, wbsa, wbsa_indexes),
    };
    tree
}
fn create_prefix_tree_from_trie_deep(
    real_node: PrefixTrie,
    wbsa: &Vec<usize>,
    wbsa_indexes: &mut WbsaIndexes,
) -> Vec<PrefixTreeNode> {
    let mut result = Vec::new();

    let rankings = real_node.get_real_rankings(wbsa, wbsa_indexes);
    if rankings.len() > 0 {
        // This Node has Rankings, so we consider it.
        let mut node = PrefixTreeNode {
            suffix_len: real_node.suffix_len,
            children: Vec::new(),
            rankings,
            min_father: real_node.min_father,
            max_father: real_node.max_father,
        };
        for (_, son) in real_node.sons {
            let nodes_list = create_prefix_tree_from_trie_deep(son, wbsa, wbsa_indexes);
            node.children.extend(nodes_list);
        }
        result.push(node);
    } else {
        // This Node is a Bridge, so we consider its Children (skipping Child Bridges).
        for (_, son) in real_node.sons {
            let nodes_list = create_prefix_tree_from_trie_deep(son, wbsa, wbsa_indexes);
            result.extend(nodes_list);
        }
    }

    result
}

// PREFIX TREE LOGGER
pub fn log_prefix_tree(prefix_tree: &PrefixTree, str: &str, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for child in &prefix_tree.children {
        log_prefix_tree_recursive(child, str, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_tree_recursive(node: &PrefixTreeNode, str: &str, file: &mut File, level: usize) {
    let mut line = format!("{}{}", " ".repeat(level), node.get_label(str));
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
        log_prefix_tree_recursive(child, str, file, level + 1);
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
