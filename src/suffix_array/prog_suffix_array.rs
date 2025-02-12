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
        child_index: usize, // FIXME: non è strano che non si usa?
        i_child: usize,
        parent_index: usize,
        i_parent: usize,
    ) {
        // FIXME: tra l'altro, controlla sempre che parent index qui sia  < child index
        // Update buffer
        let bkp = self.buffer[i_child];
        let mut i = i_child;
        while i > i_parent {
            self.buffer[i] = self.buffer[i - 1];
            i -= 1;
        }
        self.buffer[i_parent] = bkp;
        // FIXME: ...

        // Update indexes
        self.qs[parent_index] += 1;
        // FIXME: fare qualcosa se collassa con quello dopo? non credo...
        /*
        // FIXME: 2 draft
        self.qs[parent_index] -= 1;
        for curr_index in parent_index + 1..child_index {
            self.qs[curr_index] -= 1;
        }
        */
        /*
        // FIXME: 1 draft
        self.indexes_map.get_mut(&parent_index).unwrap().1 -= 1;
        for curr_index in parent_index + 1..child_index {
            let map = self.indexes_map.get_mut(&curr_index).unwrap();
            map.0 -= 1;
            map.1 -= 1;
        }
        self.indexes_map.get_mut(&child_index).unwrap().0 -= 1;
        */
    }
    pub fn update_rankings_parent_including_all_child_lss_before_curr_parent_ls(
        &mut self,
        parent_index: usize,
        curr_parent_i: usize,
        child_index: usize,
        verbose: bool,
    ) -> usize {
        // Update buffer
        // FIXME: non sare memoria ausiliaria
        let (child_p, child_q) = self.get_rankings_p_q(child_index);
        let (_, parent_q) = self.get_rankings_p_q(parent_index);
        let child_rankings_to_move = child_q - child_p;
        let mut app = Vec::with_capacity(child_rankings_to_move + parent_q - curr_parent_i);
        for child_curr_i in child_p..child_q {
            app.push(self.buffer[child_curr_i]);
        }
        for parent_curr_i in curr_parent_i..parent_q {
            app.push(self.buffer[parent_curr_i]);
        }

        if verbose {
            // FIXME: impr debug
            println!("debug: ?->{}", parent_q);
            println!("     : {} => {}", child_p, child_q);
            println!("     : {:?}", app);
        }

        let mut i = 0;
        while i < app.len() {
            self.buffer[curr_parent_i + i] = app[i];
            i += 1;
        }

        // Update indexes
        // Here we extend all Nodes Qs from This Child (excluded) down to Parent Node (included).
        let child_qs = self.qs[child_index];
        let mut succ_node_index = child_index;
        while succ_node_index > parent_index {
            self.qs[succ_node_index - 1] = child_qs;
            succ_node_index -= 1;
        }
        // Note: We are using "succ_node_index" and not "curr_node_index" since for
        // Parent Node Index=0 it would overflow in -1 and Rust doesn't like that :)

        child_rankings_to_move
    }
    pub fn update_rankings_parent_including_all_child_lss(
        &mut self,
        parent_index: usize,
        child_index: usize,
    ) -> usize {
        let (_, child_q) = self.get_rankings_p_q(child_index);

        // Update indexes
        let mut i_node_index = parent_index;
        while i_node_index <= child_index {
            self.qs[i_node_index] = child_q;
            i_node_index += 1;
        }

        // Here we return the item *next to* the last item just inherited to all Nodes involved.
        child_q
    }
    pub fn print(&self) {
        // Head
        let mut curr_p = 0;
        for i in 0..self.qs.len() {
            let q = self.qs[i];
            let num_blocks = q - curr_p;
            if num_blocks > 0 {
                print!("/{:3}", i);
                print!("{}", "    ".repeat(num_blocks - 1));
            } else {
                // FIXME: non mostrare proprio giusto?
            }
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
