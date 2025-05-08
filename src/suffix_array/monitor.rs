use crate::suffix_array::log_execution_info::round_int_100;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Monitor {
    // Timing
    pub whole_duration: MonitorInterval,
    pub p11_icfl: MonitorInterval,
    pub p12_cust_fact: MonitorInterval,
    pub p2_tree_create: MonitorInterval,
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
            p2_tree_create: MonitorInterval::new(),
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
    pub sum_duration_only_phases: Duration,
    pub whole_duration: Duration,
    // Phases
    pub p11_icfl: ExecutionTimingPhase,
    pub p12_cust_fact: ExecutionTimingPhase,
    pub p2_tree_create: ExecutionTimingPhase,
    pub p3_suffix_array: ExecutionTimingPhase,
}
impl ExecutionTiming {
    pub fn new(monitor: &Monitor) -> Self {
        let p11_icfl = monitor.p11_icfl.get_duration().unwrap();
        let p12_cust_fact = monitor.p12_cust_fact.get_duration().unwrap();
        let p2_tree_create = monitor.p2_tree_create.get_duration().unwrap();
        let p3_suffix_array = monitor.p3_suffix_array.get_duration().unwrap();
        let whole_duration = monitor.whole_duration.get_duration().unwrap();

        // Sum Durations (Only Phases)
        let sum_duration_only_phases = p11_icfl + p12_cust_fact + p2_tree_create + p3_suffix_array;

        // Percentages with extra
        /*let duration_extra = whole_duration - sum_duration_only_phases;
        let sum_micros_incl_extra = whole_duration.as_micros();*/

        // Percentages
        let sum_micros_excl_extra = sum_duration_only_phases.as_micros() as f32;
        let p11_icfl_perc = round_int_100(p11_icfl.as_micros() as f32 / sum_micros_excl_extra);
        let p12_cust_fact_perc =
            round_int_100(p12_cust_fact.as_micros() as f32 / sum_micros_excl_extra);
        let p2_tree_create_perc =
            round_int_100(p2_tree_create.as_micros() as f32 / sum_micros_excl_extra);
        let p3_suffix_array_perc =
            100 - (p11_icfl_perc + p12_cust_fact_perc + p2_tree_create_perc).min(100);
        /*let perc_p3_suffix_array = round_int_5(p3_suffix_array.as_micros() as f32 / sum_micros_excl_extra);
        let check_sum = p11_icfl_perc
            + p12_cust_fact_perc
            + p2_tree_create_perc
            + p3_suffix_array_perc
            + perc_p3_suffix_array;
        if check_sum != 100 {
            // PROBLEM
        }*/

        Self {
            sum_duration_only_phases,
            whole_duration,
            /*duration_extra,
            perc_with_extra_p11: p11_icfl.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_p12: p12_cust_fact.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_p2: p2_tree_create.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_p3: p3_suffix_array.as_micros() as f64 / sum_micros_incl_extra as f64,
            perc_with_extra_extra: duration_extra.as_micros() as f64 / sum_micros_incl_extra as f64,*/
            p11_icfl: ExecutionTimingPhase {
                dur: p11_icfl,
                perc: p11_icfl_perc,
            },
            p12_cust_fact: ExecutionTimingPhase {
                dur: p12_cust_fact,
                perc: p12_cust_fact_perc,
            },
            p2_tree_create: ExecutionTimingPhase {
                dur: p2_tree_create,
                perc: p2_tree_create_perc,
            },
            p3_suffix_array: ExecutionTimingPhase {
                dur: p3_suffix_array,
                perc: p3_suffix_array_perc,
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
