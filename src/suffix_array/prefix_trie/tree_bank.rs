pub struct TreeBank {
    node_father_data: Vec<NodeFatherData>,
}
impl TreeBank {
    pub fn new() -> Self {
        Self {
            node_father_data: Vec::new(),
        }
    }
    fn prepare_for_node_index(&mut self, node_index: usize) {
        if node_index >= self.node_father_data.len() {
            let mut next_i = self.node_father_data.len();
            while next_i <= node_index {
                self.node_father_data.push(NodeFatherData::new());
                next_i += 1;
            }
        }
    }

    // Getters
    pub fn get_node_data(&mut self, node_index: usize) -> &NodeFatherData {
        self.prepare_for_node_index(node_index);
        &self.node_father_data[node_index]
    }

    // Setters
    pub fn set_min_father(&mut self, node_index: usize, min_father: usize) {
        self.prepare_for_node_index(node_index);
        self.node_father_data[node_index].min_father = Some(min_father);
    }
    pub fn set_max_father(&mut self, node_index: usize, max_father: usize) {
        self.prepare_for_node_index(node_index);
        self.node_father_data[node_index].max_father = Some(max_father);
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
