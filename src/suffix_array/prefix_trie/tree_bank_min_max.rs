pub struct TreeBankMinMax {
    vec: Vec<TreeBankNodeMinMax>,
}
impl TreeBankMinMax {
    pub fn new(nodes_count: usize) -> Self {
        Self {
            vec: vec![TreeBankNodeMinMax::new(); nodes_count],
        }
    }
    pub fn get(&self, node_index: usize) -> &TreeBankNodeMinMax {
        &self.vec[node_index]
    }
    pub fn get_mut(&mut self, node_index: usize) -> &mut TreeBankNodeMinMax {
        &mut self.vec[node_index]
    }
}

#[derive(Clone)]
pub struct TreeBankNodeMinMax {
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl TreeBankNodeMinMax {
    pub fn new() -> Self {
        Self {
            min_father: None,
            max_father: None,
        }
    }
}
