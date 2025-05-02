pub struct TreeBankMinMax {
    vec: Vec<NodeFatherData>,
}
impl TreeBankMinMax {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }
    fn prepare_for_node_index(&mut self, node_index: usize) {
        if node_index >= self.vec.len() {
            let mut next_i = self.vec.len();
            while next_i <= node_index {
                self.vec.push(NodeFatherData::new());
                next_i += 1;
            }
        }
    }

    // Getters
    pub fn get_min_max(&mut self, node_index: usize) -> &NodeFatherData {
        self.prepare_for_node_index(node_index);
        &self.vec[node_index]
    }

    // Setters
    pub fn set_min_father(&mut self, node_index: usize, min_father: usize) {
        self.prepare_for_node_index(node_index);
        self.vec[node_index].min_father = Some(min_father);
    }
    pub fn set_max_father(&mut self, node_index: usize, max_father: usize) {
        self.prepare_for_node_index(node_index);
        self.vec[node_index].max_father = Some(max_father);
    }
}

#[derive(Clone)]
pub struct NodeFatherData {
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl NodeFatherData {
    pub fn new() -> Self {
        Self {
            min_father: None,
            max_father: None,
        }
    }
}
