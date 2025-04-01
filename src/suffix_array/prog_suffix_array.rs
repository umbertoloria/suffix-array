use std::collections::HashMap;

pub struct ProgSuffixArray {
    buffer: Vec<usize>,
    indexes_map: HashMap<usize, (usize, usize)>, // Left incl., right excl.
    indexes_rankings_forced: HashMap<usize, Vec<usize>>,
    next_index: usize,
}
impl ProgSuffixArray {
    pub fn new(str_length: usize) -> Self {
        Self {
            buffer: (0..str_length).collect::<Vec<_>>(),
            indexes_map: HashMap::new(),
            indexes_rankings_forced: HashMap::new(),
            next_index: 0,
        }
    }
    pub fn assign_rankings_to_node_index(&mut self, node_index: usize, rankings: &[usize]) {
        let mut i = self.next_index;
        for &ls_index in rankings {
            self.buffer[i] = ls_index;
            i += 1;
        }
        self.indexes_map.insert(node_index, (self.next_index, i));
        self.next_index = i;
    }
    pub fn get_rankings(&self, node_index: usize) -> &[usize] {
        if let Some(rankings_forced) = self.indexes_rankings_forced.get(&node_index) {
            &rankings_forced[0..]
        } else {
            let (p, q) = self.indexes_map.get(&node_index).unwrap();
            &self.buffer[*p..*q]
        }
    }
    pub fn save_rankings_forced(&mut self, node_index: usize, rankings_forced: Vec<usize>) {
        // FIXME: Avoid using auxiliary memory
        self.indexes_rankings_forced
            .insert(node_index, rankings_forced);
    }
}
