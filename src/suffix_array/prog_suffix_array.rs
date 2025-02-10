use std::process::exit;

pub struct ProgSuffixArray {
    buffer: Vec<usize>,
    qs: Vec<usize>,
    next_index: usize,
}
impl ProgSuffixArray {
    pub fn new(str_length: usize) -> Self {
        Self {
            buffer: (0..str_length).collect::<Vec<_>>(),
            qs: Vec::new(),
            next_index: 0,
        }
    }
    pub fn assign_rankings_to_node_index(&mut self, node_index: usize, rankings: Vec<usize>) {
        if self.qs.len() != node_index {
            exit(0x0100);
        }
        if node_index == 0 {
            self.qs.push(rankings.len());
        } else {
            let curr_last_q = self.qs[self.qs.len() - 1];
            self.qs.push(curr_last_q + rankings.len());
        }

        let mut i = self.next_index;
        for ls_index in rankings {
            self.buffer[i] = ls_index;
            i += 1;
        }
        // self.indexes_map.insert(node_index, (self.next_index, i));
        self.next_index = i;
    }
    pub fn get_rankings(&self, node_index: usize) -> &[usize] {
        /*let (p, q) = self.indexes_map.get(&node_index).unwrap(); // FIXME
        &self.buffer[*p..*q]*/
        let (p, q) = self.get_rankings_p_q(node_index);
        &self.buffer[p..q]
    }
    pub fn get_rankings_manual(&self, p: usize, q: usize) -> &[usize] {
        // FIXME: attenzione qui
        &self.buffer[p..q]
    }
    pub fn get_rankings_p_q(&self, node_index: usize) -> (usize, usize) {
        /*let (p, q) = self.indexes_map.get(&node_index).unwrap(); // FIXME: old
        (*p, *q)*/
        let q = self.qs[node_index];
        if node_index == 0 {
            (0, q)
        } else {
            (self.qs[node_index - 1], q)
        }
    }
    pub fn get_ls_index(&self, i: usize) -> usize {
        self.buffer[i]
    }
    pub fn update_rankings_child(
        &mut self,
        child_index: usize,
        i_child: usize,
        parent_index: usize,
        i_parent: usize,
    ) {
        // FIXME: tra l'altro, controlla sempre che parent index qui sia  < child index
        // Update buffer
        let bkp = self.buffer[i_parent];
        let mut i = i_parent;
        while i < i_child {
            self.buffer[i] = self.buffer[i + 1];
            i += 1;
        }
        self.buffer[i_child - 1] = bkp;
        // FIXME: ...

        // Update indexes
        self.qs[parent_index] -= 1;
        for curr_index in parent_index + 1..child_index {
            self.qs[curr_index] -= 1;
        }
        /*self.indexes_map.get_mut(&parent_index).unwrap().1 -= 1;
        for curr_index in parent_index + 1..child_index {
            let map = self.indexes_map.get_mut(&curr_index).unwrap();
            map.0 -= 1;
            map.1 -= 1;
        }
        self.indexes_map.get_mut(&child_index).unwrap().0 -= 1;*/
    }
    pub fn print(&self) {
        // Head
        let mut curr_p = 0;
        for i in 0..self.qs.len() {
            let q = self.qs[i];
            print!("/{:3}", i);
            print!("{}", "    ".repeat(q - curr_p - 1));
            curr_p = q;
        }
        println!();

        // Body
        let buffer_len = self.buffer.len();
        for i in 0..buffer_len - 1 {
            print!("|{:3}", self.buffer[i]);
        }
        println!("|{:3}|", self.buffer[buffer_len - 1]);
    }
    pub fn save_sa(self) -> Vec<usize> {
        self.buffer
    }
}
