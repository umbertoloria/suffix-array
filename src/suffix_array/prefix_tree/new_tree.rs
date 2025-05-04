use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub struct Tree {
    pub nodes: Vec<Rc<RefCell<TreeNode>>>,
}
impl Tree {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(TreeNode::new()));
        Self { nodes: vec![root] }
    }
    pub fn add(&mut self, word: &str, ranking: usize) {
        let chars_len = word.len();
        if chars_len < 1 {
            return;
        }

        let chars = word.chars().collect::<Vec<_>>();

        let mut i_node = 0;
        let mut i_char = 0;
        while i_char < chars_len {
            let curr_node_rc = self.nodes[i_node].clone();
            let mut curr_node = curr_node_rc.borrow_mut();

            let curr_char = chars[i_char];

            i_char += 1;

            // Binary Search
            let mut p = 0;
            let mut q = curr_node.children.len();
            let mut next_node_id = None;
            while p < q {
                let mid = (q + p) / 2;
                let (mid_char, mid_node_id) = curr_node.children[mid];
                if curr_char == mid_char {
                    next_node_id = Some(mid_node_id);
                    break;
                } else if curr_char < mid_char {
                    q = mid;
                } else {
                    p = mid + 1;
                }
            }
            if let Some(next_node_id) = next_node_id {
                i_node = next_node_id;
            } else {
                let new_node_id = self.create_node();
                curr_node.children.insert(p, (curr_char, new_node_id));
                i_node = new_node_id;
            }
        }
        let mut curr_node = self.nodes[i_node].borrow_mut();
        curr_node.update_rankings(ranking);
    }

    pub fn create_node(&mut self) -> usize {
        let new_node_id = self.nodes.len();
        let new_node = Rc::new(RefCell::new(TreeNode::new()));
        self.nodes.push(new_node);
        new_node_id
    }
}

pub fn log_new_tree(tree: &Tree, filepath: &str) {
    let mut file = File::create(filepath).expect("Unable to create file");
    // Logging from all First Layer Nodes to all Leafs (avoiding Root Node).
    log_new_tree_recursive(tree, 0, "", &mut file, 0);
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

    for &(char_key, child_node_id) in &node.children {
        let child_label = format!("{node_label}{char_key}");
        log_new_tree_recursive(tree, child_node_id, &child_label, file, level + 1);
    }
}

pub struct TreeNode {
    pub rankings: Vec<usize>,
    pub children: Vec<(char, usize)>,
}
impl TreeNode {
    pub fn new() -> Self {
        Self {
            rankings: Vec::new(),
            children: Vec::new(),
        }
    }
    fn update_rankings(&mut self, ranking: usize) {
        self.rankings.push(ranking);
    }
}
