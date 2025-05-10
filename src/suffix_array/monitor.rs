use crate::suffix_array::log_execution_info::round_int_100;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub whole_duration: MonitorInterval,
    pub p1_fact: MonitorInterval,
    pub p2_tree: MonitorInterval,
    pub p3_sa: MonitorInterval,

    // Values
    pub execution_outcome: ExecutionOutcome,
}
impl Monitor {
    pub fn new() -> Self {
        Self {
            whole_duration: MonitorInterval::new(),
            p1_fact: MonitorInterval::new(),
            p2_tree: MonitorInterval::new(),
            p3_sa: MonitorInterval::new(),
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

pub struct ExecutionTimingPhase {
    pub dur: Duration,
    // pub perc_with_extra: f64,
    pub perc: u16, // From 0 to 100 (sum 100).
}
pub struct ExecutionTiming {
    // These parameters are the ones used for Plotting, Execution Logging and Suite Output
    pub phases_only: Duration,
    pub whole: Duration,
    // Phases
    pub p1_fact: ExecutionTimingPhase,
    pub p2_tree: ExecutionTimingPhase,
    pub p3_sa: ExecutionTimingPhase,
}
impl ExecutionTiming {
    pub fn new(monitor: &Monitor) -> Self {
        let p1_fact = monitor.p1_fact.get_duration().unwrap();
        let p2_tree = monitor.p2_tree.get_duration().unwrap();
        let p3_sa = monitor.p3_sa.get_duration().unwrap();
        let whole_duration = monitor.whole_duration.get_duration().unwrap();

        // Sum Durations (Only Phases)
        let phases_only = p1_fact + p2_tree + p3_sa;

        // Percentages with extra
        /*let duration_extra = whole_duration - phases_only;
        let sum_micros_incl_extra = whole_duration.as_micros();*/

        // Percentages
        let sum_micros_excl_extra = phases_only.as_micros() as f32;
        let p1_fact_perc = round_int_100(p1_fact.as_micros() as f32 / sum_micros_excl_extra);
        let p2_tree_perc = round_int_100(p2_tree.as_micros() as f32 / sum_micros_excl_extra);
        let p3_sa_perc = 100 - (p1_fact_perc + p2_tree_perc).min(100);
        /*let p3_sa_perc = round_int_5(p3_sa.as_micros() as f32 / sum_micros_excl_extra);
        let check_sum = p1_fact_perc + p2_tree_perc + p3_sa_perc;
        if check_sum != 100 {
            // PROBLEM
        }*/

        Self {
            phases_only,
            whole: whole_duration,
            /*duration_extra,
            perc_with_extra_p1: p1_fact.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_p2: p2_tree.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_p3: p3_sa.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_extra: duration_extra.as_micros() as f64 / sum_micros_incl_extra as f64,*/
            p1_fact: ExecutionTimingPhase {
                dur: p1_fact,
                perc: p1_fact_perc,
            },
            p2_tree: ExecutionTimingPhase {
                dur: p2_tree,
                perc: p2_tree_perc,
            },
            p3_sa: ExecutionTimingPhase {
                dur: p3_sa,
                perc: p3_sa_perc,
            },
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
