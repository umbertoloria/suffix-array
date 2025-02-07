use crate::suffix_array::prefix_trie::PrefixTrie;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub whole_duration: MonitorInterval,
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
            whole_duration: MonitorInterval::new(),
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

    // Phase 1.1
    pub fn phase1_1_icfl_factorization_start(&mut self) {
        let now = Instant::now();
        self.p11_icfl.set_start(now);
    }
    pub fn get_phase1_1_icfl_factorization_duration(&self) -> Duration {
        self.p11_icfl.get_duration().unwrap()
    }

    // Phase 1.2
    pub fn phase1_2_custom_factorization_start(&mut self) {
        let now = Instant::now();
        self.p11_icfl.set_end(now);
        self.p12_custom.set_start(now);
    }
    pub fn get_phase1_2_custom_factorization_duration(&self) -> Duration {
        self.p12_custom.get_duration().unwrap()
    }

    // Phase 2.1
    pub fn phase2_1_prefix_trie_create_start(&mut self) {
        let now = Instant::now();
        self.p12_custom.set_end(now);
        self.p21_trie_create.set_start(now);
    }
    pub fn phase2_1_prefix_trie_create_stop(&mut self) {
        let now = Instant::now();
        self.p21_trie_create.set_end(now);
    }
    pub fn get_phase2_1_prefix_trie_create_duration(&self) -> Duration {
        self.p21_trie_create.get_duration().unwrap()
    }

    // Phase 2.2
    pub fn phase2_2_prefix_trie_merge_rankings_start(&mut self) {
        let now = Instant::now();
        self.p22_trie_merge_rankings.set_start(now);
    }
    pub fn phase2_2_prefix_trie_merge_rankings_stop(&mut self) {
        let now = Instant::now();
        self.p22_trie_merge_rankings.set_end(now);
    }
    pub fn get_phase2_2_prefix_trie_merge_rankings_duration(&self) -> Duration {
        self.p22_trie_merge_rankings.get_duration().unwrap()
    }

    // Phase 2.3
    pub fn phase2_3_prefix_trie_in_prefix_merge_start(&mut self) {
        let now = Instant::now();
        self.p23_trie_in_prefix_merge.set_start(now);
    }
    pub fn phase2_3_prefix_trie_in_prefix_merge_stop(&mut self) {
        let now = Instant::now();
        self.p23_trie_in_prefix_merge.set_end(now);
    }
    pub fn get_phase2_3_prefix_trie_in_prefix_merge_duration(&self) -> Duration {
        self.p23_trie_in_prefix_merge.get_duration().unwrap()
    }

    // Phase 2.4
    pub fn phase2_4_prefix_tree_create_start(&mut self) {
        let now = Instant::now();
        self.p24_tree_create.set_start(now);
    }
    pub fn phase2_4_prefix_tree_create_stop(&mut self) {
        let now = Instant::now();
        self.p24_tree_create.set_end(now);
    }
    pub fn get_phase2_4_prefix_tree_create_duration(&self) -> Duration {
        self.p24_tree_create.get_duration().unwrap()
    }

    // Phase 3
    pub fn phase3_suffix_array_compose_start(&mut self) {
        let now = Instant::now();
        self.p3_sa_compose.set_start(now);
    }
    pub fn phase3_suffix_array_compose_stop(&mut self) {
        let now = Instant::now();
        self.p3_sa_compose.set_end(now);
    }
    pub fn get_phase3_suffix_array_compose_duration(&self) -> Duration {
        self.p3_sa_compose.get_duration().unwrap()
    }

    // Whole Process
    pub fn process_start(&mut self) {
        let now = Instant::now();
        self.whole_duration.set_start(now);
    }
    pub fn process_end(&mut self) {
        let now = Instant::now();
        self.whole_duration.set_end(now);
    }

    pub fn get_whole_process_duration_included_extra(&self) -> Duration {
        self.whole_duration.get_duration().unwrap()
    }
    pub fn get_extra_time_spent(&self) -> Duration {
        let whole = self.get_whole_process_duration_included_extra();
        whole - self.get_sum_phases_duration()
    }
    pub fn get_sum_phases_duration(&self) -> Duration {
        let p11 = self.p11_icfl.get_duration().unwrap();
        let p12 = self.p12_custom.get_duration().unwrap();
        let p21 = self.p21_trie_create.get_duration().unwrap();
        let p22 = self.p22_trie_merge_rankings.get_duration().unwrap();
        let p23 = self.p23_trie_in_prefix_merge.get_duration().unwrap();
        let p24 = self.p24_tree_create.get_duration().unwrap();
        let p3 = self.p3_sa_compose.get_duration().unwrap();
        p11 + p12 + p21 + p22 + p23 + p24 + p3
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

// MONITOR LOGGER
pub fn log_monitor_after_process_ended(monitor: &Monitor, filepath: String) {
    let mut content = String::new();

    content.push_str(&format_duration(
        " > Duration phases                ",
        &monitor.get_sum_phases_duration(),
        None,
    ));
    content.push_str(&format_duration(
        " > Duration (with extra)          ",
        &monitor.get_whole_process_duration_included_extra(),
        None,
    ));

    let duration_p11 = monitor.get_phase1_1_icfl_factorization_duration();
    let duration_p12 = monitor.get_phase1_2_custom_factorization_duration();
    let duration_p21 = monitor.get_phase2_1_prefix_trie_create_duration();
    let duration_p22 = monitor.get_phase2_2_prefix_trie_merge_rankings_duration();
    let duration_p23 = monitor.get_phase2_3_prefix_trie_in_prefix_merge_duration();
    let duration_p24 = monitor.get_phase2_4_prefix_tree_create_duration();
    let duration_p3 = monitor.get_phase3_suffix_array_compose_duration();
    let durations = vec![
        duration_p11,
        duration_p12,
        duration_p21,
        duration_p22,
        duration_p23,
        duration_p24,
        duration_p3,
    ];
    let mut sum_micros = 0;
    for duration in durations {
        sum_micros += duration.as_micros();
    }
    let percentage_p11 = (duration_p11.as_micros() as f64 / sum_micros as f64) * 100.0;
    let percentage_p12 = (duration_p12.as_micros() as f64 / sum_micros as f64) * 100.0;
    let percentage_p21 = (duration_p21.as_micros() as f64 / sum_micros as f64) * 100.0;
    let percentage_p22 = (duration_p22.as_micros() as f64 / sum_micros as f64) * 100.0;
    let percentage_p23 = (duration_p23.as_micros() as f64 / sum_micros as f64) * 100.0;
    let percentage_p24 = (duration_p24.as_micros() as f64 / sum_micros as f64) * 100.0;
    let percentage_p3 = (duration_p3.as_micros() as f64 / sum_micros as f64) * 100.0;

    content.push_str(&format_duration(
        " > Phase 1.1: Factorization ICFL  ",
        &duration_p11,
        Some(percentage_p11),
    ));
    content.push_str(&format_duration(
        " > Phase 1.2: Factorization Custom",
        &duration_p12,
        Some(percentage_p12),
    ));
    content.push_str(&format_duration(
        " > Phase 2.1: Trie Create         ",
        &duration_p21,
        Some(percentage_p21),
    ));
    content.push_str(&format_duration(
        " > Phase 2.2: Trie Merge rankings ",
        &duration_p22,
        Some(percentage_p22),
    ));
    content.push_str(&format_duration(
        " > Phase 2.3: Trie In-prefix merge",
        &duration_p23,
        Some(percentage_p23),
    ));
    content.push_str(&format_duration(
        " > Phase 2.4: Tree create         ",
        &duration_p24,
        Some(percentage_p24),
    ));
    content.push_str(&format_duration(
        " > Phase 3  : Suffix Array        ",
        &duration_p3,
        Some(percentage_p3),
    ));

    let mut file = File::create(filepath).expect("Unable to create file");
    file.write(content.as_bytes())
        .expect("Unable to write line");
    file.flush().expect("Unable to flush file");
}

fn format_duration(prefix: &str, duration: &Duration, percentage: Option<f64>) -> String {
    let mut result = String::new();

    result.push_str(&format!(
        "{}: {:15} micros / {:15.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64(),
    ));
    if let Some(percentage) = percentage {
        result.push_str(&format!(" / {:7.3}%\n", percentage));
    } else {
        result.push_str(&"\n");
    }

    result
}
