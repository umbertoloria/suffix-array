pub struct TreeBank {
    node_father_data: Vec<NodeFatherData>,
}
impl TreeBank {
    pub fn new(nodes_count: usize) -> Self {
        Self {
            node_father_data: vec![NodeFatherData::new(); nodes_count],
        }
    }

    // Getters
    pub fn get_node_data(&self, node_index: usize) -> &NodeFatherData {
        &self.node_father_data[node_index]
    }

    // Setters
    pub fn set_min_father(&mut self, node_index: usize, min_father: usize) {
        self.node_father_data[node_index].min_father = Some(min_father);
    }
    pub fn set_max_father(&mut self, node_index: usize, max_father: usize) {
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
