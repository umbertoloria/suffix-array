use crate::suffix_array::log_execution_info::round_int_100;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub whole_duration: MonitorInterval,
    pub p11_icfl: MonitorInterval,
    pub p12_custom: MonitorInterval,
    pub p21_trie_create: MonitorInterval,
    pub p22_trie_merge_rankings: MonitorInterval,
    pub p24_tree_in_prefix_merge: MonitorInterval,
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
            p24_tree_in_prefix_merge: MonitorInterval::new(),
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

    // FIXME: Fix phase names

    // Phase 2.4
    pub fn phase2_4_prefix_tree_in_prefix_merge_start(&mut self) {
        let now = Instant::now();
        self.p24_tree_in_prefix_merge.set_start(now);
    }
    pub fn phase2_4_prefix_tree_in_prefix_merge_stop(&mut self) {
        let now = Instant::now();
        self.p24_tree_in_prefix_merge.set_end(now);
    }
    pub fn get_phase2_4_prefix_tree_in_prefix_merge_duration(&self) -> Duration {
        self.p24_tree_in_prefix_merge.get_duration().unwrap()
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

    // EVALUATE TIMING AND PROPORTIONS
    pub fn transform_info_execution_info(self) -> ExecutionInfo {
        ExecutionInfo {
            execution_timing: self.transform_into_execution_timing(),
            execution_outcome: self.transform_into_execution_outcome(),
        }
    }
    fn transform_into_execution_timing(&self) -> ExecutionTiming {
        ExecutionTiming::new(
            self.get_phase1_1_icfl_factorization_duration(),
            self.get_phase1_2_custom_factorization_duration(),
            self.get_phase2_1_prefix_trie_create_duration(),
            self.get_phase2_2_prefix_trie_merge_rankings_duration(),
            self.get_phase2_4_prefix_tree_in_prefix_merge_duration(),
            self.get_phase3_suffix_array_compose_duration(),
            self.get_whole_process_duration_included_extra(),
        )
    }
    fn transform_into_execution_outcome(&self) -> ExecutionOutcome {
        ExecutionOutcome {
            compares_with_two_cfs: self.compares_with_two_cfs,
            compares_with_one_cf: self.compares_with_one_cf,
            compares_using_rules: self.compares_using_rules,
            compares_using_strcmp: self.compares_using_strcmp,
        }
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

// MONITOR LOGGER
pub struct ExecutionInfo {
    pub execution_timing: ExecutionTiming,
    pub execution_outcome: ExecutionOutcome,
}
pub struct ExecutionTiming {
    pub duration_p11: Duration,
    pub duration_p12: Duration,
    pub duration_p21: Duration,
    pub duration_p22: Duration,
    pub duration_p24: Duration,
    pub duration_p3: Duration,
    pub duration_extra: Duration,
    pub sum_duration_only_phases: Duration,
    pub whole_duration: Duration,
    pub prop_with_extra_p11: f64,
    pub prop_with_extra_p12: f64,
    pub prop_with_extra_p21: f64,
    pub prop_with_extra_p22: f64,
    pub prop_with_extra_p24: f64,
    pub prop_with_extra_p3: f64,
    pub prop_with_extra_extra: f64,
    // Those are from 0 to 100 (sum 100).
    pub prop_p11: u16,
    pub prop_p12: u16,
    pub prop_p21: u16,
    pub prop_p22: u16,
    pub prop_p24: u16,
    pub prop_p3: u16,
}
impl ExecutionTiming {
    pub fn new(
        duration_p11: Duration,
        duration_p12: Duration,
        duration_p21: Duration,
        duration_p22: Duration,
        duration_p24: Duration,
        duration_p3: Duration,
        whole_duration: Duration,
    ) -> Self {
        // Sum Durations (Only Phases)
        let sum_duration_only_phases =
            duration_p11 + duration_p12 + duration_p21 + duration_p22 + duration_p24 + duration_p3;

        // Extra Duration
        let duration_extra = whole_duration - sum_duration_only_phases;

        // Proportions (with extra)
        let sum_micros_incl_extra = whole_duration.as_micros();
        let prop_with_extra_p11 = duration_p11.as_micros() as f64 / sum_micros_incl_extra as f64;
        let prop_with_extra_p12 = duration_p12.as_micros() as f64 / sum_micros_incl_extra as f64;
        let prop_with_extra_p21 = duration_p21.as_micros() as f64 / sum_micros_incl_extra as f64;
        let prop_with_extra_p22 = duration_p22.as_micros() as f64 / sum_micros_incl_extra as f64;
        let prop_with_extra_p24 = duration_p24.as_micros() as f64 / sum_micros_incl_extra as f64;
        let prop_with_extra_p3 = duration_p3.as_micros() as f64 / sum_micros_incl_extra as f64;
        let prop_with_extra_extra =
            duration_extra.as_micros() as f64 / sum_micros_incl_extra as f64;

        let sum_micros_excl_extra = sum_duration_only_phases.as_micros() as f32;
        let prop_p11 = round_int_100(duration_p11.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p12 = round_int_100(duration_p12.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p21 = round_int_100(duration_p21.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p22 = round_int_100(duration_p22.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p24 = round_int_100(duration_p24.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p3 = 100 - (prop_p11 + prop_p12 + prop_p21 + prop_p22 + prop_p24).min(100);
        /*let prop_p3 = round_int_5(duration_p3.as_micros() as f32 / sum_micros_excl_extra);
        let check_sum = prop_p11 + prop_p12 + prop_p21 + prop_p22 + prop_p24 + prop_p3;
        if check_sum != 100 {
            // PROBLEM
        }*/

        Self {
            duration_p11,
            duration_p12,
            duration_p21,
            duration_p22,
            duration_p24,
            duration_p3,
            sum_duration_only_phases,
            whole_duration,
            duration_extra,
            prop_with_extra_p11,
            prop_with_extra_p12,
            prop_with_extra_p21,
            prop_with_extra_p22,
            prop_with_extra_p24,
            prop_with_extra_p3,
            prop_with_extra_extra,
            prop_p11,
            prop_p12,
            prop_p21,
            prop_p22,
            prop_p24,
            prop_p3,
        }
    }
}
pub struct ExecutionOutcome {
    pub compares_with_two_cfs: usize,
    pub compares_with_one_cf: usize,
    pub compares_using_rules: usize,
    pub compares_using_strcmp: usize,
}
impl ExecutionOutcome {
    pub fn print(&self) {
        println!("Execution Outcome:");
        println!(" > two custom: {}", self.compares_with_two_cfs);
        println!(" > one custom: {}", self.compares_with_one_cf);
        println!(" > rules: {}", self.compares_using_rules);
        println!(" > string compares: {}", self.compares_using_strcmp);
    }
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
