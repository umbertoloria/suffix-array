use crate::suffix_array::prefix_trie::PrefixTrie;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub p11_icfl: MonitorInterval,
    pub p12_custom: MonitorInterval,
    pub p21_trie_create: MonitorInterval,
    pub p22_trie_merge_rankings: MonitorInterval,
    pub p23_trie_in_prefix_merge: MonitorInterval,
    pub p24_tree_create: MonitorInterval,
    pub p3_sa_compose: MonitorInterval,

    // Values
    pub compares_with_two_cfs: usize,
    pub compares_with_one_cf: usize,
    pub compares_using_rules: usize,
    pub compares_using_strcmp: usize,
}
impl Monitor {
    pub fn new() -> Self {
        Self {
            p11_icfl: MonitorInterval::new(),
            p12_custom: MonitorInterval::new(),
            p21_trie_create: MonitorInterval::new(),
            p22_trie_merge_rankings: MonitorInterval::new(),
            p23_trie_in_prefix_merge: MonitorInterval::new(),
            p24_tree_create: MonitorInterval::new(),
            p3_sa_compose: MonitorInterval::new(),
            compares_with_two_cfs: 0,
            compares_with_one_cf: 0,
            compares_using_rules: 0,
            compares_using_strcmp: 0,
        }
    }

    pub fn phase1_1_icfl_factorization_start(&mut self) {
        self.p11_icfl.set_start(Instant::now());
    }
    pub fn get_phase1_1_icfl_factorization_duration(&self) -> Duration {
        self.p11_icfl.get_duration().unwrap()
    }
    pub fn phase1_2_custom_factorization_start(&mut self) {
        let now = Instant::now();
        self.p11_icfl.set_end(now);
        self.p12_custom.set_start(now);
    }
    pub fn get_phase1_2_custom_factorization_duration(&self) -> Duration {
        self.p12_custom.get_duration().unwrap()
    }
    pub fn phase2_1_prefix_trie_create_start(&mut self) {
        let now = Instant::now();
        self.p12_custom.set_end(now);
        self.p21_trie_create.set_start(now);
    }
    pub fn get_phase2_1_prefix_trie_create_duration(&self) -> Duration {
        self.p21_trie_create.get_duration().unwrap()
    }
    pub fn phase2_2_prefix_trie_merge_rankings_start(&mut self) {
        let now = Instant::now();
        self.p21_trie_create.set_end(now);
        self.p22_trie_merge_rankings.set_start(now);
    }
    pub fn get_phase2_2_prefix_trie_merge_rankings_duration(&self) -> Duration {
        self.p22_trie_merge_rankings.get_duration().unwrap()
    }
    pub fn phase2_3_prefix_trie_in_prefix_merge_start(&mut self) {
        let now = Instant::now();
        self.p22_trie_merge_rankings.set_end(now);
        self.p23_trie_in_prefix_merge.set_start(now);
    }
    pub fn get_phase2_3_prefix_trie_in_prefix_merge_duration(&self) -> Duration {
        self.p23_trie_in_prefix_merge.get_duration().unwrap()
    }
    pub fn phase2_4_prefix_tree_create_start(&mut self) {
        let now = Instant::now();
        self.p23_trie_in_prefix_merge.set_end(now);
        self.p24_tree_create.set_start(now);
    }
    pub fn get_phase2_4_prefix_tree_create_duration(&self) -> Duration {
        self.p24_tree_create.get_duration().unwrap()
    }
    pub fn phase3_suffix_array_compose_start(&mut self) {
        let now = Instant::now();
        self.p24_tree_create.set_end(now);
        self.p3_sa_compose.set_start(now);
    }
    pub fn get_phase3_suffix_array_compose_duration(&self) -> Duration {
        self.p3_sa_compose.get_duration().unwrap()
    }

    pub fn process_end(&mut self) {
        let now = Instant::now();
        self.p3_sa_compose.set_end(now);
    }
    pub fn get_process_duration(&self) -> Option<Duration> {
        if let Some(begin) = self.p11_icfl.start {
            if let Some(end) = self.p3_sa_compose.end {
                return Some(end - begin);
            }
        }
        None
    }

    pub fn new_compare_of_two_ls_in_custom_factors(&mut self) {
        self.compares_with_two_cfs += 1;
    }
    pub fn new_compare_one_ls_in_custom_factor(&mut self) {
        self.compares_with_one_cf += 1;
    }
    pub fn new_compare_using_rules(&mut self) {
        self.compares_using_rules += 1;
    }
    pub fn new_compare_using_actual_string_compare(&mut self) {
        self.compares_using_strcmp += 1;
    }
    pub fn print(&self) {
        println!("Monitor output:");
        println!(" > two custom: {}", self.compares_with_two_cfs);
        println!(" > one custom: {}", self.compares_with_one_cf);
        println!(" > rules: {}", self.compares_using_rules);
        println!(" > string compares: {}", self.compares_using_strcmp);
    }
}

#[derive(Debug)]
pub struct MonitorInterval {
    pub start: Option<Instant>,
    pub end: Option<Instant>,
}
impl MonitorInterval {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }
    pub fn set_start(&mut self, start: Instant) {
        self.start = Some(start);
    }
    pub fn set_end(&mut self, end: Instant) {
        self.end = Some(end);
    }
    pub fn get_duration(&self) -> Option<Duration> {
        if let Some(start) = self.start {
            if let Some(end) = self.end {
                return Some(end - start);
            }
        }
        None
    }
}

// PREFIX TRIE LOGGER
pub fn log_prefix_trie(prefix_trie: &PrefixTrie, wbsa: &Vec<usize>, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for (_, son) in &prefix_trie.sons {
        log_prefix_trie_recursive(son, wbsa, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_trie_recursive(node: &PrefixTrie, wbsa: &Vec<usize>, file: &mut File, level: usize) {
    let mut line = format!("{}{}", " ".repeat(level), node.label);
    let mut rankings = node.get_real_rankings(wbsa);
    if !rankings.is_empty() {
        line.push_str(" [");
        let last_ranking = rankings.pop().unwrap();
        for ranking in rankings {
            line.push_str(&format!("{}, ", ranking));
        }
        line.push_str(&format!("{}]", last_ranking));
    }
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for (_, son) in &node.sons {
        log_prefix_trie_recursive(son, wbsa, file, level + 1);
    }
}
