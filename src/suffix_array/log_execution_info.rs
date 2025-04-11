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
                duration_phases_______________: et.sum_duration_only_phases.as_micros(),
                duration_phases_with_extra____: et.whole_duration.as_micros(),
                phase_1_1_factorization_icfl__: et.duration_p11.as_micros(),
                phase_1_2_factorization_custom: et.duration_p12.as_micros(),
                phase_2_1_trie_create_________: et.duration_p21.as_micros(),
                phase_2_2_trie_shrink_________: et.duration_p22.as_micros(),
                phase_2_3_trie_merge_rankings_: et.duration_p23.as_micros(),
                phase_2_4_trie_in_prefix_merge: et.duration_p24.as_micros(),
                phase_3_0_suffix_array________: et.duration_p3.as_micros(),
            },
            seconds: ExecutionInfoFileFormatSeconds {
                duration_phases_______________: round_secs_x_xxx(et.sum_duration_only_phases),
                duration_phases_with_extra____: round_secs_x_xxx(et.whole_duration),
                phase_1_1_factorization_icfl__: round_secs_x_xxx(et.duration_p11),
                phase_1_2_factorization_custom: round_secs_x_xxx(et.duration_p12),
                phase_2_1_trie_create_________: round_secs_x_xxx(et.duration_p21),
                phase_2_2_trie_shrink_________: round_secs_x_xxx(et.duration_p22),
                phase_2_3_trie_merge_rankings_: round_secs_x_xxx(et.duration_p23),
                phase_2_4_trie_in_prefix_merge: round_secs_x_xxx(et.duration_p24),
                phase_3_0_suffix_array________: round_secs_x_xxx(et.duration_p3),
            },
            percentages: ExecutionInfoFileFormatPercentages {
                phase_1_1_factorization_icfl__: et.prop_p11,
                phase_1_2_factorization_custom: et.prop_p12,
                phase_2_1_trie_create_________: et.prop_p21,
                phase_2_2_trie_shrink_________: et.prop_p22,
                phase_2_3_trie_merge_rankings_: et.prop_p23,
                phase_2_4_trie_in_prefix_merge: et.prop_p24,
                phase_3_0_suffix_array________: et.prop_p3,
            },
        }
    }
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatMicros {
    duration_phases_______________: u128,
    duration_phases_with_extra____: u128,
    phase_1_1_factorization_icfl__: u128,
    phase_1_2_factorization_custom: u128,
    phase_2_1_trie_create_________: u128,
    phase_2_2_trie_shrink_________: u128,
    phase_2_3_trie_merge_rankings_: u128,
    phase_2_4_trie_in_prefix_merge: u128,
    phase_3_0_suffix_array________: u128,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatSeconds {
    duration_phases_______________: f32,
    duration_phases_with_extra____: f32,
    phase_1_1_factorization_icfl__: f32,
    phase_1_2_factorization_custom: f32,
    phase_2_1_trie_create_________: f32,
    phase_2_2_trie_shrink_________: f32,
    phase_2_3_trie_merge_rankings_: f32,
    phase_2_4_trie_in_prefix_merge: f32,
    phase_3_0_suffix_array________: f32,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatPercentages {
    phase_1_1_factorization_icfl__: u16,
    phase_1_2_factorization_custom: u16,
    phase_2_1_trie_create_________: u16,
    phase_2_2_trie_shrink_________: u16,
    phase_2_3_trie_merge_rankings_: u16,
    phase_2_4_trie_in_prefix_merge: u16,
    phase_3_0_suffix_array________: u16,
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
