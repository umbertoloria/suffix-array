use crate::suffix_array::monitor::ExecutionTiming;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct ExecutionInfoFileFormat {
    micros: ExecutionInfoFileFormatMicros,
    seconds: ExecutionInfoFileFormatSeconds,
    percentages: ExecutionInfoFileFormatPercentages,
}
impl ExecutionInfoFileFormat {
    pub fn new(et: &ExecutionTiming) -> Self {
        Self {
            micros: ExecutionInfoFileFormatMicros {
                duration_phases: et.phases_only.as_micros(),
                phase_1_fact___: et.p1_fact.dur.as_micros(),
                phase_2_tree___: et.p2_tree.dur.as_micros(),
                phase_3_sa_____: et.p3_sa.dur.as_micros(),
                duration_phases_with_extra: et.whole.as_micros(),
            },
            seconds: ExecutionInfoFileFormatSeconds {
                duration_phases: round_secs_x_xxx(et.phases_only),
                phase_1_fact___: round_secs_x_xxx(et.p1_fact.dur),
                phase_2_tree___: round_secs_x_xxx(et.p2_tree.dur),
                phase_3_sa_____: round_secs_x_xxx(et.p3_sa.dur),
                duration_phases_with_extra: round_secs_x_xxx(et.whole),
            },
            percentages: ExecutionInfoFileFormatPercentages {
                phase_1_fact: et.p1_fact.perc,
                phase_2_tree: et.p2_tree.perc,
                phase_3_sa__: et.p3_sa.perc,
            },
        }
    }
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatMicros {
    duration_phases: u128,
    phase_1_fact___: u128,
    phase_2_tree___: u128,
    phase_3_sa_____: u128,
    duration_phases_with_extra: u128,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatSeconds {
    duration_phases: f32,
    phase_1_fact___: f32,
    phase_2_tree___: f32,
    phase_3_sa_____: f32,
    duration_phases_with_extra: f32,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatPercentages {
    phase_1_fact: u16,
    phase_2_tree: u16,
    phase_3_sa__: u16,
}

fn round_secs_x_xxx(duration: Duration) -> f32 {
    round_f64_c(duration.as_secs_f64(), 3)
}
fn round_f64_c(value: f64, decimal_digits: u32) -> f32 {
    let mover = 10_i32.pow(decimal_digits) as f64;
    ((value * mover).round() / mover) as f32
}

pub fn round_int_100(value: f32) -> u16 {
    let mover = 100.0;
    (value * mover).round() as u16
}
