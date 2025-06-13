use crate::files::fasta::get_fasta_content;
use crate::files::paths::get_path_in_generated_folder;
use crate::plot::plot::draw_plot_from_monitor;
use crate::suffix_array::classic_suffix_array::compute_classic_suffix_array;
use crate::suffix_array::monitor::ExecutionInfo;
use crate::suffix_array::new_suffix_array::compute_innovative_suffix_array;
use std::time::Duration;

// SUITE COMPLETE FOR CLASSIC VS INNOVATIVE COMPUTATION
pub fn suite_complete_on_fasta_file(
    fasta_file_name: &str,
    chunk_size_vec: &Vec<Option<usize>>,
    max_duration_in_micros: u32,
    log_execution: bool,
    log_fact: bool,
    log_trees_and_suffix_array: bool,
) {
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", fasta_file_name);

    // READING FILE
    let src_str = &get_fasta_content(get_path_in_generated_folder(fasta_file_name));

    // CLASSIC SUFFIX ARRAY
    let classic_suffix_array_computation = compute_classic_suffix_array(src_str);
    let classic_suffix_array = classic_suffix_array_computation.suffix_array;
    let classic_sa_computation_duration = classic_suffix_array_computation.duration;
    println!("CLASSIC SUFFIX ARRAY CALCULATION");
    println!(
        " > Duration: {:15} micros",
        classic_sa_computation_duration.as_micros(),
    );
    println!(
        " > Duration: {:15.3} seconds",
        classic_sa_computation_duration.as_secs_f64(),
    );
    // println!(" > Suffix Array: {:?}", classic_suffix_array);

    // INNOVATIVE SUFFIX ARRAY
    println!();
    println!("INNOVATIVE SUFFIX ARRAY CALCULATION");
    let mut chunk_size_and_phase_micros_list = Vec::new();
    for &chunk_size in chunk_size_vec {
        let test_result = run_and_validate_test(
            &classic_suffix_array,
            fasta_file_name,
            src_str,
            chunk_size,
            log_execution,
            log_fact,
            log_trees_and_suffix_array,
        );
        if test_result.failed {
            break;
        }
        let et = &test_result.execution_info.execution_timing;
        let micros = (
            et.p1_fact.dur.as_micros() as u64,
            et.p2_tree.dur.as_micros() as u64,
            et.p3_sa.dur.as_micros() as u64,
        );
        let chunk_size_or_zero = chunk_size.unwrap_or(0);
        chunk_size_and_phase_micros_list.push((chunk_size_or_zero, micros));
    }

    // Plots
    draw_plot_from_monitor(
        fasta_file_name,
        classic_sa_computation_duration,
        chunk_size_and_phase_micros_list,
        max_duration_in_micros,
    );
}

pub struct RunAndValidateTestOutput {
    execution_info: ExecutionInfo,
    failed: bool,
}
fn run_and_validate_test(
    classic_suffix_array: &Vec<usize>,
    fasta_file_name: &str,
    src_str: &str,
    chunk_size: Option<usize>,
    log_execution: bool,
    log_fact: bool,
    log_trees_and_suffix_array: bool,
) -> RunAndValidateTestOutput {
    let innovative_suffix_array_computation = compute_innovative_suffix_array(
        fasta_file_name,
        src_str,
        chunk_size,
        log_execution,
        log_fact,
        log_trees_and_suffix_array,
    );
    let suffix_array = innovative_suffix_array_computation.suffix_array;
    let execution_info = innovative_suffix_array_computation.execution_info;

    let execution_timing = &execution_info.execution_timing;
    let execution_outcome = &execution_info.execution_outcome;

    let chunk_size_or_zero = chunk_size.unwrap_or(0);
    println!("[CHUNK SIZE={chunk_size_or_zero}]");
    print_duration(" > Duration phases        ", &execution_timing.phases_only);
    print_duration(" > Phase 1: Factorization ", &execution_timing.p1_fact.dur);
    print_duration(" > Phase 2: Tree          ", &execution_timing.p2_tree.dur);
    print_duration(" > Phase 3: Suffix Array  ", &execution_timing.p3_sa.dur);
    print_duration(" > Duration (with extra)  ", &execution_timing.whole);
    if cfg!(feature = "verbose") {
        execution_outcome.print();
    }
    // println!(" > Suffix Array: {:?}", wbsa);

    // VERIFICATION
    let mut success = true;
    if suffix_array.len() != classic_suffix_array.len() {
        success = false;
        println!("Wanna Be Suffix Array is insufficient in size");
    } else {
        let mut i = 0;
        while i < classic_suffix_array.len() {
            let sa_item = classic_suffix_array[i];
            let wbsa_item = suffix_array[i];
            if wbsa_item != sa_item {
                println!("Wanna Be Suffix Array is insufficient: element [{}] should be \"{}\" but is \"{}\"", i, sa_item, wbsa_item);
                success = false;
            }
            i += 1;
        }
    }

    let mut result = RunAndValidateTestOutput {
        failed: false,
        execution_info,
    };

    if success {
        // println!("Wanna Be Suffix Array is PERFECT :)");
    } else {
        println!(" > Suffix Array: {:?}", suffix_array);
        println!("Wanna Be Suffix Array is WRONG!!! :(");
        result.failed = true;
    }

    result
}

fn print_duration(prefix: &str, duration: &Duration) {
    println!(
        "{}: {:15} micros / {:15.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64()
    );
}
