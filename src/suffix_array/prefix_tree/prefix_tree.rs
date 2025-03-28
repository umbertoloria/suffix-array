pub struct PrefixTree {
    pub children: Vec<PrefixTreeNode>,
}
pub struct PrefixTreeNode {
    pub index: usize,
    pub suffix_len: usize,
    pub children: Vec<PrefixTreeNode>,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
