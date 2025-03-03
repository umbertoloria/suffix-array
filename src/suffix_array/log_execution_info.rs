use crate::suffix_array::monitor::ExecutionTiming;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormat {
    micros: ExecutionInfoFileFormatMicros,
    seconds: ExecutionInfoFileFormatSeconds,
    percentages: ExecutionInfoFileFormatPercentages,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatMicros {
    duration_phases_______________: u128,
    duration_phases_with_extra____: u128,
    phase_1_1_factorization_icfl__: u128,
    phase_1_2_factorization_custom: u128,
    phase_2_1_trie_create_________: u128,
    phase_2_2_trie_merge_rankings_: u128,
    phase_2_3_trie_in_prefix_merge: u128,
    phase_2_4_tree_create_________: u128,
    phase_3_0_suffix_array________: u128,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatSeconds {
    duration_phases_______________: f32,
    duration_phases_with_extra____: f32,
    phase_1_1_factorization_icfl__: f32,
    phase_1_2_factorization_custom: f32,
    phase_2_1_trie_create_________: f32,
    phase_2_2_trie_merge_rankings_: f32,
    phase_2_3_trie_in_prefix_merge: f32,
    phase_2_4_tree_create_________: f32,
    phase_3_0_suffix_array________: f32,
}
#[derive(Serialize, Deserialize)]
struct ExecutionInfoFileFormatPercentages {
    phase_1_1_factorization_icfl__: f32,
    phase_1_2_factorization_custom: f32,
    phase_2_1_trie_create_________: f32,
    phase_2_2_trie_merge_rankings_: f32,
    phase_2_3_trie_in_prefix_merge: f32,
    phase_2_4_tree_create_________: f32,
    phase_3_0_suffix_array________: f32,
}

pub fn log_execution_timing(et: &ExecutionTiming, filepath: String) {
    let file_format = ExecutionInfoFileFormat {
        micros: ExecutionInfoFileFormatMicros {
            duration_phases_______________: et.sum_duration_only_phases.as_micros(),
            duration_phases_with_extra____: et.whole_duration.as_micros(),
            phase_1_1_factorization_icfl__: et.duration_p11.as_micros(),
            phase_1_2_factorization_custom: et.duration_p12.as_micros(),
            phase_2_1_trie_create_________: et.duration_p21.as_micros(),
            phase_2_2_trie_merge_rankings_: et.duration_p22.as_micros(),
            phase_2_3_trie_in_prefix_merge: et.duration_p23.as_micros(),
            phase_2_4_tree_create_________: et.duration_p24.as_micros(),
            phase_3_0_suffix_array________: et.duration_p3.as_micros(),
        },
        seconds: ExecutionInfoFileFormatSeconds {
            duration_phases_______________: round_custom_3(
                et.sum_duration_only_phases.as_secs_f64(),
            ),
            duration_phases_with_extra____: round_custom_3(et.whole_duration.as_secs_f64()),
            phase_1_1_factorization_icfl__: round_custom_3(et.duration_p11.as_secs_f64()),
            phase_1_2_factorization_custom: round_custom_3(et.duration_p12.as_secs_f64()),
            phase_2_1_trie_create_________: round_custom_3(et.duration_p21.as_secs_f64()),
            phase_2_2_trie_merge_rankings_: round_custom_3(et.duration_p22.as_secs_f64()),
            phase_2_3_trie_in_prefix_merge: round_custom_3(et.duration_p23.as_secs_f64()),
            phase_2_4_tree_create_________: round_custom_3(et.duration_p24.as_secs_f64()),
            phase_3_0_suffix_array________: round_custom_3(et.duration_p3.as_secs_f64()),
        },
        percentages: ExecutionInfoFileFormatPercentages {
            phase_1_1_factorization_icfl__: round_custom_3(et.prop_p11 * 100.0),
            phase_1_2_factorization_custom: round_custom_3(et.prop_p12 * 100.0),
            phase_2_1_trie_create_________: round_custom_3(et.prop_p21 * 100.0),
            phase_2_2_trie_merge_rankings_: round_custom_3(et.prop_p22 * 100.0),
            phase_2_3_trie_in_prefix_merge: round_custom_3(et.prop_p23 * 100.0),
            phase_2_4_tree_create_________: round_custom_3(et.prop_p24 * 100.0),
            phase_3_0_suffix_array________: round_custom_3(et.prop_p3 * 100.0),
        },
    };
    let json = serde_json::to_string_pretty(&file_format).unwrap();
    let mut file = File::create(filepath).expect("Unable to create file");
    file.write(json.as_bytes())
        .expect("Unable to write JSON string");
    file.flush().expect("Unable to flush file");
}

fn round_custom_3(value: f64) -> f32 {
    round_custom_decimal_digits(value, 3)
}
fn round_custom_decimal_digits(value: f64, decimal_digits: u32) -> f32 {
    let mover: f64 = 10_i32.pow(decimal_digits) as f64;
    let result = (value * mover).round() / mover;
    result as f32
}
