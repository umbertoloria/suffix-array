use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    children: HashMap<char, Box<Node>>,
    is_leaf: bool,
}
impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_leaf: false,
        }
    }
    fn print(&self, tab: usize) {
        for value in &self.children.iter().collect::<Vec<_>>() {
            println!("{}└--* {}", "|  ".repeat(tab), value.0);
            value.1.print(tab + 1);
        }
    }
}

#[derive(Debug)]
struct SuffixTree {
    root: Node,
}
impl SuffixTree {
    fn new() -> Self {
        SuffixTree { root: Node::new() }
    }

    fn insert(&mut self, s: &str) {
        let mut node = &mut self.root;
        for (i, c) in s.chars().enumerate() {
            if !node.children.contains_key(&c) {
                node.children.insert(c, Box::new(Node::new()));
            }
            node = node.children.get_mut(&c).unwrap();
            if i == s.len() - 1 {
                node.is_leaf = true;
            }
        }
    }

    fn print(&self, tab: usize) {
        println!("|");
        self.root.print(tab);
    }
}

pub fn main_suffix() {
    let mut tree = SuffixTree::new();
    tree.insert("umberto");
    tree.insert("roberto");
    tree.insert("umbria");
    tree.insert("roberta");
    tree.print(0);
}
