pub struct NodeFatherBank {
    vec: Vec<NodeFatherData>,
}
impl NodeFatherBank {
    pub fn new(nodes_count: usize) -> Self {
        Self {
            vec: vec![NodeFatherData::new(); nodes_count],
        }
    }

    // Getters
    pub fn get_node_data(&self, node_index: usize) -> &NodeFatherData {
        &self.vec[node_index]
    }

    // Setters
    pub fn set_min_father(&mut self, node_index: usize, min_father: usize) {
        self.vec[node_index].min_father = Some(min_father);
    }
    pub fn set_max_father(&mut self, node_index: usize, max_father: usize) {
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
