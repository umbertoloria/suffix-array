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
                phase_1_1_factorization_icfl__: et.p11_icfl.as_micros(),
                phase_1_2_factorization_custom: et.p12_cust_fact.as_micros(),
                phase_2_1_trie_create_________: et.p21_trie_create.as_micros(),
                phase_2_2_shrink______________: et.p22_shrink.as_micros(),
                phase_2_3_merge_rankings______: et.p23_merge_rankings.as_micros(),
                phase_2_4_in_prefix_merge_____: et.p24_in_prefix_merge.as_micros(),
                phase_3_0_suffix_array________: et.p3_suffix_array.as_micros(),
            },
            seconds: ExecutionInfoFileFormatSeconds {
                duration_phases_______________: round_secs_x_xxx(et.sum_duration_only_phases),
                duration_phases_with_extra____: round_secs_x_xxx(et.whole_duration),
                phase_1_1_factorization_icfl__: round_secs_x_xxx(et.p11_icfl),
                phase_1_2_factorization_custom: round_secs_x_xxx(et.p12_cust_fact),
                phase_2_1_trie_create_________: round_secs_x_xxx(et.p21_trie_create),
                phase_2_2_shrink______________: round_secs_x_xxx(et.p22_shrink),
                phase_2_3_merge_rankings______: round_secs_x_xxx(et.p23_merge_rankings),
                phase_2_4_in_prefix_merge_____: round_secs_x_xxx(et.p24_in_prefix_merge),
                phase_3_0_suffix_array________: round_secs_x_xxx(et.p3_suffix_array),
            },
            percentages: ExecutionInfoFileFormatPercentages {
                phase_1_1_factorization_icfl__: et.prop_p11_icfl,
                phase_1_2_factorization_custom: et.prop_p12_cust_fact,
                phase_2_1_trie_create_________: et.prop_p21_trie_create,
                phase_2_2_shrink______________: et.prop_p22_shrink,
                phase_2_3_merge_rankings______: et.prop_p23_merge_rankings,
                phase_2_4_in_prefix_merge_____: et.prop_p24_in_prefix_merge,
                phase_3_0_suffix_array________: et.prop_p3_suffix_array,
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
    phase_2_2_shrink______________: u128,
    phase_2_3_merge_rankings______: u128,
    phase_2_4_in_prefix_merge_____: u128,
    phase_3_0_suffix_array________: u128,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatSeconds {
    duration_phases_______________: f32,
    duration_phases_with_extra____: f32,
    phase_1_1_factorization_icfl__: f32,
    phase_1_2_factorization_custom: f32,
    phase_2_1_trie_create_________: f32,
    phase_2_2_shrink______________: f32,
    phase_2_3_merge_rankings______: f32,
    phase_2_4_in_prefix_merge_____: f32,
    phase_3_0_suffix_array________: f32,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatPercentages {
    phase_1_1_factorization_icfl__: u16,
    phase_1_2_factorization_custom: u16,
    phase_2_1_trie_create_________: u16,
    phase_2_2_shrink______________: u16,
    phase_2_3_merge_rankings______: u16,
    phase_2_4_in_prefix_merge_____: u16,
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
