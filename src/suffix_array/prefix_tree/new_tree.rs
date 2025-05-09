use crate::suffix_array::chunking::get_max_factor_size;
use crate::suffix_array::monitor::Monitor;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub fn create_new_tree<'a>(
    s_bytes: &'a [u8],
    custom_indexes: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    monitor: &mut Monitor,
    verbose: bool,
) -> Tree<'a> {
    let str_length = s_bytes.len();
    let max_factor_size =
        get_max_factor_size(&custom_indexes, str_length).expect("max_factor_size is not valid");

    let mut tree = Tree::new();

    let custom_indexes_len = custom_indexes.len();
    let last_factor_size = str_length - custom_indexes[custom_indexes_len - 1];

    let mut params_canonical = Vec::new();
    let mut params_custom = Vec::new();

    for ls_size in 1..max_factor_size + 1 {
        // Every iteration looks for all Custom Factors whose length is <= "ls_size" and, if there
        // exist, takes their Local Suffixes of "ls_size" length.

        // Last Factor
        if ls_size <= last_factor_size {
            let ls_index = str_length - ls_size;
            let is_custom_ls = is_custom_vec[ls_index];
            if is_custom_ls {
                params_custom.push((ls_index, ls_size));
            } else {
                params_canonical.push((ls_index, ls_size));
            }
        }

        // All Factors from first to second-last
        for i_factor in 0..custom_indexes_len - 1 {
            let curr_factor_size = custom_indexes[i_factor + 1] - custom_indexes[i_factor];
            if ls_size <= curr_factor_size {
                let ls_index = custom_indexes[i_factor + 1] - ls_size;
                let is_custom_ls = is_custom_vec[ls_index];
                if is_custom_ls {
                    params_custom.push((ls_index, ls_size));
                } else {
                    params_canonical.push((ls_index, ls_size));
                }
            }
        }

        // LSs that come from Canonical Factors (already sorted)
        for &(ls_index, ls_size) in &params_canonical {
            tree.add(ls_index, ls_size, false, s_bytes, verbose);
            if verbose {
                tree.print();
            }
        }
        params_canonical.clear();

        // LSs that come from Custom Factors (to sort)
        for &(ls_index, ls_size) in &params_custom {
            tree.add(ls_index, ls_size, true, s_bytes, verbose);
            if verbose {
                tree.print();
            }
        }
        params_custom.clear();
    }

    tree
}

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
        let mut i_node = 0;
        let mut i_char = 0;
        while i_char < ls_size {
            let curr_node_rc = self.nodes[i_node].clone();
            let mut curr_node = curr_node_rc.borrow_mut();

            let rest_of_ls = &s_bytes[ls_index + i_char..ls_index + ls_size];

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
                // i_char += rest_of_ls.len(); // Here useless but meaningful.
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
                    // Due to how this New Tree structure is used, all the code commented below can
                    // be simplified in this code here. Still, it's kept for further explanation.
                    i_node = mid_node_id;
                    i_char += i;
                    break;

                    /*
                    // TOD: Simplify branch nidification
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
                            // TOD: Here, we should update this Edge (that has string "mid_str")
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
                    */
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

    // PRINT
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

// LOGGER
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

// STRING ABSTRACTION
type PrefixTrieString = [u8];
fn get_string_clone(str_type: &PrefixTrieString) -> String {
    // TODO: Needs cloning
    let cloned_vec = str_type.to_vec();
    String::from_utf8(cloned_vec).unwrap()
}
