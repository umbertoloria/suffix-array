use crate::suffix_array::prefix_trie::PrefixTrie;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub begin: Option<Instant>,
    pub end: Option<Instant>,

    // Values
    pub compares_with_two_cfs: usize,
    pub compares_with_one_cf: usize,
    pub compares_using_rules: usize,
    pub compares_using_strcmp: usize,
}
impl Monitor {
    pub fn new() -> Self {
        Self {
            begin: None,
            end: None,
            compares_with_two_cfs: 0,
            compares_with_one_cf: 0,
            compares_using_rules: 0,
            compares_using_strcmp: 0,
        }
    }

    pub fn process_begin(&mut self) {
        self.begin = Some(Instant::now());
    }
    pub fn process_end(&mut self) {
        self.end = Some(Instant::now());
    }
    pub fn get_process_duration(&self) -> Option<Duration> {
        if let Some(begin) = self.begin {
            if let Some(end) = self.end {
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
            line.push_str(format!("{}, ", ranking).as_str());
        }
        line.push_str(format!("{}]", last_ranking).as_str());
    }
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for (_, son) in &node.sons {
        log_prefix_trie_recursive(son, wbsa, file, level + 1);
    }
}
