use crate::suffix_array::log_execution_info::round_int_100;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub whole_duration: MonitorInterval,
    pub p11_icfl: MonitorInterval,
    pub p12_cust_fact: MonitorInterval,
    pub p21_trie_create: MonitorInterval,
    pub p22_shrink: MonitorInterval,
    pub p23_merge_rankings: MonitorInterval,
    pub p24_in_prefix_merge: MonitorInterval,
    pub p3_suffix_array: MonitorInterval,

    // Values
    pub execution_outcome: ExecutionOutcome,
}
impl Monitor {
    pub fn new() -> Self {
        Self {
            whole_duration: MonitorInterval::new(),
            p11_icfl: MonitorInterval::new(),
            p12_cust_fact: MonitorInterval::new(),
            p21_trie_create: MonitorInterval::new(),
            p22_shrink: MonitorInterval::new(),
            p23_merge_rankings: MonitorInterval::new(),
            p24_in_prefix_merge: MonitorInterval::new(),
            p3_suffix_array: MonitorInterval::new(),
            execution_outcome: ExecutionOutcome::new(),
        }
    }

    // EXECUTION OUTCOME
    pub fn new_compare_of_two_ls_in_custom_factors(&mut self) {
        self.execution_outcome.compares_with_two_cfs += 1;
    }
    pub fn new_compare_one_ls_in_custom_factor(&mut self) {
        self.execution_outcome.compares_with_one_cf += 1;
    }
    pub fn new_compare_using_rules(&mut self) {
        self.execution_outcome.compares_using_rules += 1;
    }
    pub fn new_compare_using_actual_string_compare(&mut self) {
        self.execution_outcome.compares_using_strcmp += 1;
    }

    // EVALUATE TIMING AND PROPORTIONS
    pub fn transform_info_execution_info(self) -> ExecutionInfo {
        ExecutionInfo {
            execution_timing: ExecutionTiming::new(&self),
            execution_outcome: self.execution_outcome,
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
    pub fn start(&mut self) {
        let now = Instant::now();
        self.start = Some(now);
    }
    pub fn stop(&mut self) {
        let now = Instant::now();
        self.end = Some(now);
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

pub struct ExecutionInfo {
    pub execution_timing: ExecutionTiming,
    pub execution_outcome: ExecutionOutcome,
}

pub struct ExecutionTiming {
    // These parameters are the ones used for Plotting, Execution Logging and Suite Output
    pub p11_icfl: Duration,
    pub p12_cust_fact: Duration,
    pub p21_trie_create: Duration,
    pub p22_shrink: Duration,
    pub p23_merge_rankings: Duration,
    pub p24_in_prefix_merge: Duration,
    pub p3_suffix_array: Duration,
    pub sum_duration_only_phases: Duration,
    pub whole_duration: Duration,
    // pub prop_with_extra_p11: f64,
    // pub prop_with_extra_p12: f64,
    // pub prop_with_extra_p21: f64,
    // pub prop_with_extra_p22: f64,
    // pub prop_with_extra_p23: f64,
    // pub prop_with_extra_p24: f64,
    // pub prop_with_extra_p3: f64,
    // pub prop_with_extra_extra: f64,
    // Those are from 0 to 100 (sum 100).
    pub prop_p11_icfl: u16,
    pub prop_p12_cust_fact: u16,
    pub prop_p21_trie_create: u16,
    pub prop_p22_shrink: u16,
    pub prop_p23_merge_rankings: u16,
    pub prop_p24_in_prefix_merge: u16,
    pub prop_p3_suffix_array: u16,
}
impl ExecutionTiming {
    pub fn new(monitor: &Monitor) -> Self {
        let p11_icfl = monitor.p11_icfl.get_duration().unwrap();
        let p12_cust_fact = monitor.p12_cust_fact.get_duration().unwrap();
        let p21_trie_create = monitor.p21_trie_create.get_duration().unwrap();
        let p22_shrink = monitor.p22_shrink.get_duration().unwrap();
        let p23_merge_rankings = monitor.p23_merge_rankings.get_duration().unwrap();
        let p24_in_prefix_merge = monitor.p24_in_prefix_merge.get_duration().unwrap();
        let p3_suffix_array = monitor.p3_suffix_array.get_duration().unwrap();
        let whole_duration = monitor.whole_duration.get_duration().unwrap();

        // Sum Durations (Only Phases)
        let sum_duration_only_phases = p11_icfl
            + p12_cust_fact
            + p21_trie_create
            + p22_shrink
            + p23_merge_rankings
            + p24_in_prefix_merge
            + p3_suffix_array;

        // Proportions (with extra)
        /*let duration_extra = whole_duration - sum_duration_only_phases;
        let sum_micros_incl_extra = whole_duration.as_micros();*/

        // Props
        let sum_micros_excl_extra = sum_duration_only_phases.as_micros() as f32;
        let prop_p11_icfl = round_int_100(p11_icfl.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p12_cust_fact =
            round_int_100(p12_cust_fact.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p21_trie_create =
            round_int_100(p21_trie_create.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p22_shrink = round_int_100(p22_shrink.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p23_merge_rankings =
            round_int_100(p23_merge_rankings.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p24_in_prefix_merge =
            round_int_100(p24_in_prefix_merge.as_micros() as f32 / sum_micros_excl_extra);
        let prop_p3_suffix_array = 100
            - (prop_p11_icfl
                + prop_p12_cust_fact
                + prop_p21_trie_create
                + prop_p22_shrink
                + prop_p23_merge_rankings
                + prop_p24_in_prefix_merge)
                .min(100);
        /*let prop_p3 = round_int_5(p3_suffix_array.as_micros() as f32 / sum_micros_excl_extra);
        let check_sum = prop_p11_icfl
            + prop_p12_cust_fact
            + prop_p21_trie_create
            + prop_p22_shrink
            + prop_p23_merge_rankings
            + prop_p24_in_prefix_merge
            + prop_p3;
        if check_sum != 100 {
            // PROBLEM
        }*/

        Self {
            p11_icfl,
            p12_cust_fact,
            p21_trie_create,
            p22_shrink,
            p23_merge_rankings,
            p24_in_prefix_merge,
            p3_suffix_array,
            sum_duration_only_phases,
            whole_duration,
            /*duration_extra,
            prop_with_extra_p11: p11_icfl.as_micros() as f64 / sum_micros_incl_extra as f64,
            prop_with_extra_p12: p12_cust_fact.as_micros() as f64 / sum_micros_incl_extra as f64,
            prop_with_extra_p21: p21_trie_create.as_micros() as f64 / sum_micros_incl_extra as f64,
            prop_with_extra_p22: p22_shrink.as_micros() as f64 / sum_micros_incl_extra as f64,
            prop_with_extra_p23: p23_merge_rankings.as_micros() as f64
                / sum_micros_incl_extra as f64,
            prop_with_extra_p24: p24_in_prefix_merge.as_micros() as f64
                / sum_micros_incl_extra as f64,
            prop_with_extra_p3: p3_suffix_array.as_micros() as f64 / sum_micros_incl_extra as f64,
            prop_with_extra_extra: duration_extra.as_micros() as f64 / sum_micros_incl_extra as f64,*/
            prop_p11_icfl,
            prop_p12_cust_fact,
            prop_p21_trie_create,
            prop_p22_shrink,
            prop_p23_merge_rankings,
            prop_p24_in_prefix_merge,
            prop_p3_suffix_array,
        }
    }
}

#[derive(Debug)]
pub struct ExecutionOutcome {
    pub compares_with_two_cfs: usize,
    pub compares_with_one_cf: usize,
    pub compares_using_rules: usize,
    pub compares_using_strcmp: usize,
}
impl ExecutionOutcome {
    pub fn new() -> Self {
        Self {
            compares_with_two_cfs: 0,
            compares_with_one_cf: 0,
            compares_using_rules: 0,
            compares_using_strcmp: 0,
        }
    }
    pub fn print(&self) {
        println!("Execution Outcome:");
        println!(" > two custom: {}", self.compares_with_two_cfs);
        println!(" > one custom: {}", self.compares_with_one_cf);
        println!(" > rules: {}", self.compares_using_rules);
        println!(" > string compares: {}", self.compares_using_strcmp);
    }
}
