use crate::suffix_array::prefix_trie::prefix_trie::{get_string_clone, PrefixTrieString};
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

const ROOT_ID: usize = 0;
pub struct Tree<'a> {
    nodes: Vec<Rc<RefCell<TreeNode<'a>>>>,
}
impl<'a> Tree<'a> {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(TreeNode::new(0)));
        Self { nodes: vec![root] }
    }
    pub fn get_root(&self) -> &Rc<RefCell<TreeNode<'a>>> {
        &self.nodes[ROOT_ID]
    }
    pub fn get_node(&self, index: usize) -> &Rc<RefCell<TreeNode<'a>>> {
        &self.nodes[index]
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
                    " -> i_char={i_char} on REST={}, i_node={i_node}, ls_index={ls_index}",
                    get_string_clone(rest_of_ls)
                );
            }

            if curr_node.children.is_empty() {
                let new_node_id = self.create_node(ls_size);
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
                            println!("     -> Case 1: found final Node={mid_node_id}");
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
                            if verbose {
                                println!(
                                    "       -> Case 2A: mid_str={} prefix of rest_of_ls={}",
                                    get_string_clone(mid_str),
                                    get_string_clone(rest_of_ls),
                                );
                            }
                            // Case 2A. We have that "mid_str" is prefix of "rest_of_ls".
                            i_node = mid_node_id;
                            i_char += i;
                            break;
                        } else {
                            // Then it's "i < mid_str.len()".
                            // Case 2B. We have that "rest_of_ls" is prefix of "mid_str".
                            if verbose {
                                println!(
                                    "       -> Case 2B: rest_of_ls={} prefix of mid_str={}",
                                    get_string_clone(rest_of_ls),
                                    get_string_clone(mid_str),
                                );
                            }
                            // TODO: Here, we should update this Edge (that has string "mid_str")
                            //  to use the string "rest_of_ls", since "rest_of_ls" is prefix of
                            //  "mid_str". Here we avoid coding this since the callee of this
                            //  function already knows that has to add first all LSs with length "i"
                            //  and then all LSs with length "i+1", so this case should never happen
                            //  (if this ordering is followed).
                            // Example: In the Tree there was "\0" [] -> "CA" [10, 4], and now we
                            // want to insert a node "C" for ranking 6.
                            // The result should be "\0" [] -> "C" [6] -> "CA" [10, 4].
                            // What we have here instead is: "\0" [] -> "CA" [10, 4]
                            //  and "\0" [] -> "C" [6], so "CA" and "C" are siblings. But actually
                            // "C" should be inserted as the *new father* of "CA" (even if "CA" was
                            // already there).
                            let new_node_id = self.create_node(i_char + rest_of_ls.len());
                            curr_node.children.insert(mid, (rest_of_ls, new_node_id));
                            i_node = new_node_id;
                            i_char += rest_of_ls.len();
                            break;
                        }
                    }
                }
            }
            if p >= q {
                if verbose {
                    println!("     -> found index p={p} for creating new node");
                }
                let new_node_id = self.create_node(i_char + rest_of_ls.len());
                curr_node.children.insert(p, (rest_of_ls, new_node_id));
                i_node = new_node_id;
                i_char += rest_of_ls.len();
            }
        }
        if verbose {
            println!("   -> Populating node id={i_node} with new ranking {ls_index}");
        }
        let mut curr_node = self.nodes[i_node].borrow_mut();
        curr_node.update_rankings(ls_index, is_custom_ls, s_bytes);
    }

    pub fn create_node(&mut self, suffix_len: usize) -> usize {
        let new_node_id = self.nodes.len();
        let new_node = Rc::new(RefCell::new(TreeNode::new(suffix_len)));
        self.nodes.push(new_node);
        new_node_id
    }

    pub fn get_nodes_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn save_rankings_on_prog_sa(&mut self, prog_sa: &mut ProgSuffixArray) {
        self.save_node_rankings_into_prog_sa(ROOT_ID, prog_sa);
    }
    fn save_node_rankings_into_prog_sa(&mut self, node_id: usize, prog_sa: &mut ProgSuffixArray) {
        let node_rc = self.get_node(node_id).clone();
        let mut node = node_rc.borrow_mut();
        prog_sa.set_rankings_to_node(node_id, &mut node.rankings);

        for &(_, child_node_id) in &node.children {
            self.save_node_rankings_into_prog_sa(child_node_id, prog_sa);
        }
    }

    pub fn print(&self) {
        self.print_node(ROOT_ID, 0, "");
    }
    fn print_node(&self, self_node_id: usize, tabs_offset: usize, self_label: &str) {
        let self_node = self.get_node(self_node_id).borrow();
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            self_label,
            format!("{:?}", self_node.rankings),
        );
        for &(child_node_prefix, child_node_id) in &self_node.children {
            let prefix_str = get_string_clone(child_node_prefix);
            let child_node_label = format!("{}{}", self_label, prefix_str);
            self.print_node(child_node_id, tabs_offset + 1, &child_node_label);
        }
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
        // node_id, // Avoid showing Node ID.
        "",
    );
    let node = tree.get_node(node_id).borrow();
    let rankings = &node.rankings;
    line.push_str(" [");
    for i in 0..rankings.len() - 1 {
        let ranking = rankings[i];
        line.push_str(&format!("{}, ", ranking));
    }
    line.push_str(&format!("{}]", rankings[rankings.len() - 1]));
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for &(child_node_prefix, child_node_id) in &node.children {
        let child_label = format!("{}{}", node_label, get_string_clone(child_node_prefix));
        log_new_tree_recursive(tree, child_node_id, &child_label, file, level + 1);
    }
}
pub fn log_new_tree_using_prog_sa(tree: &Tree, filepath: String, prog_sa: &ProgSuffixArray) {
    let mut file = File::create(filepath).expect("Unable to create file");
    // Logging from all First Layer Nodes to all Leafs (avoiding Root Node).
    for &(child_node_prefix, child_node_id) in &tree.get_root().borrow().children {
        let child_label = format!("{}", get_string_clone(child_node_prefix));
        log_new_tree_using_prog_sa_recursive(
            tree,
            child_node_id,
            &child_label,
            prog_sa,
            &mut file,
            0,
        );
    }
    file.flush().expect("Unable to flush file");
}
fn log_new_tree_using_prog_sa_recursive(
    tree: &Tree,
    node_id: usize,
    node_label: &str,
    prog_sa: &ProgSuffixArray,
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
    let node = tree.get_node(node_id).borrow();
    let rankings = prog_sa.get_rankings(node_id);
    line.push_str(" [");
    for i in 0..rankings.len() - 1 {
        let ranking = rankings[i];
        line.push_str(&format!("{}, ", ranking));
    }
    line.push_str(&format!("{}]", rankings[rankings.len() - 1]));
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for &(child_node_prefix, child_node_id) in &node.children {
        let child_label = format!("{}{}", node_label, get_string_clone(child_node_prefix));
        log_new_tree_using_prog_sa_recursive(
            tree,
            child_node_id,
            &child_label,
            prog_sa,
            file,
            level + 1,
        );
    }
}

pub struct TreeNode<'a> {
    pub suffix_len: usize,
    pub rankings: Vec<usize>,
    pub children: Vec<(&'a PrefixTrieString, usize)>,
}
impl<'a> TreeNode<'a> {
    pub fn new(suffix_len: usize) -> Self {
        Self {
            suffix_len,
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
