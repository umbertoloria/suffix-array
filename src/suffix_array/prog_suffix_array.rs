use std::collections::HashMap;

pub struct ProgSuffixArray {
    buffer: Vec<usize>,
    indexes_map: HashMap<usize, (usize, usize)>, // Left incl., right excl.
    next_index: usize,
}
impl ProgSuffixArray {
    pub fn new(str_length: usize) -> Self {
        Self {
            buffer: (0..str_length).collect::<Vec<_>>(),
            indexes_map: HashMap::new(),
            next_index: 0,
        }
    }
    pub fn assign_rankings_to_node_index(&mut self, node_index: usize, rankings: &Vec<usize>) {
        let mut i = self.next_index;
        for &ls_index in rankings {
            self.buffer[i] = ls_index;
            i += 1;
        }
        self.indexes_map.insert(node_index, (self.next_index, i));
        self.next_index = i;
    }
    pub fn get_rankings(&self, node_index: usize) -> &[usize] {
        let (p, q) = self.indexes_map.get(&node_index).unwrap();
        &self.buffer[*p..*q]
    }
}
