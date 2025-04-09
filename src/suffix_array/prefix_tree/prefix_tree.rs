pub struct PrefixTree {
    pub children: Vec<PrefixTreeNode>,
}
pub struct PrefixTreeNode {
    pub index: usize,
    pub suffix_len: usize,
    pub children: Vec<PrefixTreeNode>,
}
