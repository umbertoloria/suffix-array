use crate::suffix_array::prefix_trie::prefix_trie::{get_string_clone, PrefixTrieString};
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub struct Tree<'a> {
    pub nodes: Vec<Rc<RefCell<TreeNode<'a>>>>,
}
impl<'a> Tree<'a> {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        Self { nodes: vec![root] }
    }
    pub fn get_root(&self) -> &Rc<RefCell<TreeNode<'a>>> {
        &self.nodes[0]
    }
    pub fn add(
        &mut self,
        ls_index: usize,
        ls_size: usize,
        is_custom_ls: bool,
        s_bytes: &'a [u8],
        verbose: bool,
    ) {
        let ls = &s_bytes[ls_index..ls_index + ls_size];

        let mut i_node = 0;
        let mut i_char = 0;
        while i_char < ls_size {
            let curr_node_rc = self.nodes[i_node].clone();
            let mut curr_node = curr_node_rc.borrow_mut();

            let rest_of_ls = &ls[i_char..];

            if verbose {
                println!(
                    " -> i_char={i_char} on REST={}, i_node={i_node}",
                    get_string_clone(rest_of_ls)
                );
            }

            if curr_node.children.is_empty() {
                let new_node_id = self.create_node();
                curr_node.children.push((rest_of_ls, new_node_id));
                i_node = new_node_id;
                if verbose {
                    println!(
                        "   -> was empty, so created node id={new_node_id} with prefix={}",
                        get_string_clone(rest_of_ls)
                    );
                }
                break;
            }

            // Binary Search
            let mut p = 0;
            let mut q = curr_node.children.len();
            while p < q {
                let mid = (q + p) / 2;
                if verbose {
                    println!("   -> Binary Search: considering Mid Index={mid}");
                }
                let (mid_str, mid_node_id) = curr_node.children[mid];

                // Comparing "Mid. Str." with "Rest of LS".
                let mut i = 0;
                while i < rest_of_ls.len() && i < mid_str.len() {
                    if rest_of_ls[i] != mid_str[i] {
                        break;
                    }
                    i += 1;
                }
                if i < rest_of_ls.len() && i < mid_str.len() {
                    if verbose {
                        println!("     -> try another element");
                    }
                    // Strings are different.
                    if rest_of_ls[i] < mid_str[i] {
                        q = mid;
                    } else {
                        // Then it's "rest_of_ls[i] > mid_str[i]".
                        p = mid + 1;
                    }
                } else {
                    // TODO: Simplify branch nidification
                    // Here it can be true either:
                    // 1. Strings are the same, or
                    // 2. Strings have some prefix relation.
                    if i == rest_of_ls.len() && i == mid_str.len() {
                        // Case 1. Strings are the same.
                        if verbose {
                            println!("     -> Case 1: next node={mid_node_id}");
                        }
                        i_node = mid_node_id;
                        i_char += i;
                        break;
                    } else {
                        if verbose {
                            println!("     -> Case 2");
                        }
                        // Case 2. It can be either:
                        // 2A. "mid_str" is prefix of "rest_of_ls", or
                        // 2B. "rest_of_ls" is prefix of "mid_str".
                        if i < rest_of_ls.len() {
                            // Case 2A. We have that "mid_str" is prefix of "rest_of_ls".
                            i_node = mid_node_id;
                            i_char += i;
                            break;
                        } else {
                            // Then it's "i < mid_str.len()".
                            // Case 2B. We have that "rest_of_ls" is prefix of "mid_str".
                            let new_node_id = self.create_node();
                            curr_node.children.insert(mid, (rest_of_ls, new_node_id));
                            i_node = new_node_id;
                            i_char += i;
                            break;
                        }
                    }
                }
            }
            if p >= q {
                if verbose {
                    println!("     -> found index p={p} for creating new node");
                }
                let new_node_id = self.create_node();
                curr_node.children.insert(p, (rest_of_ls, new_node_id));
                i_node = new_node_id;
                i_char += rest_of_ls.len();
            }
        }
        if verbose {
            println!("   -> Populating node id={i_node} with new ranking");
        }
        let mut curr_node = self.nodes[i_node].borrow_mut();
        curr_node.update_rankings(ls_index, is_custom_ls, s_bytes);
    }

    pub fn create_node(&mut self) -> usize {
        let new_node_id = self.nodes.len();
        let new_node = Rc::new(RefCell::new(TreeNode::new()));
        self.nodes.push(new_node);
        new_node_id
    }
}

pub fn log_new_tree(tree: &Tree, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    // Logging from all First Layer Nodes to all Leafs (avoiding Root Node).
    for &(child_node_prefix, child_node_id) in &tree.get_root().borrow().children {
        let child_label = format!("{}", get_string_clone(child_node_prefix));
        log_new_tree_recursive(tree, child_node_id, &child_label, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_new_tree_recursive(
    tree: &Tree,
    node_id: usize,
    node_label: &str,
    file: &mut File,
    level: usize,
) {
    let mut line = format!(
        //
        "{}{} <{}>",
        " ".repeat(level),
        node_label,
        node_id,
    );
    let node = tree.nodes[node_id].borrow();
    let rankings = &node.rankings;
    if !rankings.is_empty() {
        // This Node has Rankings.
        line.push_str(" [");
        for i in 0..rankings.len() - 1 {
            let ranking = rankings[i];
            line.push_str(&format!("{}, ", ranking));
        }
        line.push_str(&format!("{}]", rankings[rankings.len() - 1]));
    }
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");

    for &(child_node_prefix, child_node_id) in &node.children {
        let child_label = format!("{}{}", node_label, get_string_clone(child_node_prefix));
        log_new_tree_recursive(tree, child_node_id, &child_label, file, level + 1);
    }
}

pub struct TreeNode<'a> {
    pub rankings: Vec<usize>,
    pub children: Vec<(&'a PrefixTrieString, usize)>,
}
impl<'a> TreeNode<'a> {
    pub fn new() -> Self {
        Self {
            rankings: Vec::new(),
            children: Vec::new(),
        }
    }
    fn update_rankings(&mut self, ls_index: usize, is_custom_ls: bool, s_bytes: &[u8]) {
        if is_custom_ls {
            let custom_gs = &s_bytes[ls_index..];
            let idx = self.rankings.partition_point(|&gs_index| {
                let gs = &s_bytes[gs_index..];
                // TODO: Monitor string compare
                gs <= custom_gs
            });
            self.rankings.insert(idx, ls_index);
            // Duplicated code: look for (*njk).
        } else {
            self.rankings.push(ls_index);
        }
    }
}
