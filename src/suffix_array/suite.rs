use crate::files::fasta::get_fasta_content;
use crate::files::paths::get_path_in_generated_folder;
use crate::plot::plot::draw_plot_from_monitor;
use crate::suffix_array::classic_suffix_array::compute_classic_suffix_array;
use crate::suffix_array::monitor::ExecutionInfo;
use crate::suffix_array::new_suffix_array::{compute_innovative_suffix_array, DebugMode};
use std::time::Duration;

// SUITE COMPLETE FOR CLASSIC VS INNOVATIVE COMPUTATION
pub fn suite_complete_on_fasta_file(
    fasta_file_name: &str,
    chunk_size_interval: (usize, usize), // Both incl.
    perform_logging: bool,
    debug_mode: DebugMode,
) {
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", fasta_file_name);

    // READING FILE
    let src_str = &get_fasta_content(get_path_in_generated_folder(fasta_file_name));

    // CLASSIC SUFFIX ARRAY
    let classic_suffix_array_computation = compute_classic_suffix_array(src_str, false);
    let classic_suffix_array = classic_suffix_array_computation.suffix_array;
    println!("CLASSIC SUFFIX ARRAY CALCULATION");
    println!(
        " > Duration: {:15} micros",
        classic_suffix_array_computation.duration.as_micros()
    );
    println!(
        " > Duration: {:15.3} seconds",
        classic_suffix_array_computation.duration.as_secs_f64()
    );
    // println!(" > Suffix Array: {:?}", classic_suffix_array);

    // INNOVATIVE SUFFIX ARRAY
    println!();
    println!("INNOVATIVE SUFFIX ARRAY CALCULATION");
    let chunks_interval = (chunk_size_interval.0..chunk_size_interval.1 + 1).collect::<Vec<_>>();
    let mut chunk_size_and_execution_info_list = Vec::new();
    /*if run_also_without_custom_factorization {
        // TODO: Lethal with medium files on small PCs
        run_and_validate_test(
            fasta_file_name,
            perform_logging,
            debug_mode,
            src_str,
            &classic_suffix_array,
            None,
        );
        // TODO: Add into to plot
    }*/
    for &chunk_size in &chunks_interval {
        let test_result = run_and_validate_test(
            fasta_file_name,
            perform_logging,
            debug_mode,
            src_str,
            &classic_suffix_array,
            Some(chunk_size),
        );
        chunk_size_and_execution_info_list.push((chunk_size, test_result.execution_info));
        if test_result.failed {
            break;
        }
    }

    // Plots
    draw_plot_from_monitor(fasta_file_name, chunk_size_and_execution_info_list);
}

pub struct RunAndValidateTestOutput {
    execution_info: ExecutionInfo,
    failed: bool,
}
fn run_and_validate_test(
    fasta_file_name: &str,
    perform_logging: bool,
    debug_mode: DebugMode,
    src_str: &str,
    classic_suffix_array: &Vec<usize>,
    chunk_size: Option<usize>,
) -> RunAndValidateTestOutput {
    let innovative_suffix_array_computation = compute_innovative_suffix_array(
        fasta_file_name,
        src_str,
        chunk_size,
        perform_logging,
        debug_mode,
    );
    let suffix_array = innovative_suffix_array_computation.suffix_array;
    let execution_info = innovative_suffix_array_computation.execution_info;

    let execution_timing = &execution_info.execution_timing;
    let execution_outcome = &execution_info.execution_outcome;
    let chunk_size_or_zero = if let Some(chunk_size) = chunk_size {
        chunk_size
    } else {
        0
    };

    if chunk_size_or_zero > 0 {
        println!("[CHUNK SIZE={chunk_size_or_zero}]");
    } else {
        println!("[NO CHUNKING]");
    }
    print_duration(
        " > Duration phases                ",
        &execution_timing.sum_duration_only_phases,
    );
    print_duration(
        " > Duration (with extra)          ",
        &execution_timing.whole_duration,
    );
    print_duration(
        " > Phase 1.1: Factorization ICFL  ",
        &execution_timing.duration_p11,
    );
    print_duration(
        " > Phase 1.2: Factorization Custom",
        &execution_timing.duration_p12,
    );
    print_duration(
        " > Phase 2.1: Trie Create         ",
        &execution_timing.duration_p21,
    );
    print_duration(
        " > Phase 2.2: Trie Merge rankings ",
        &execution_timing.duration_p22,
    );
    print_duration(
        " > Phase 2.3: Trie In-prefix merge",
        &execution_timing.duration_p23,
    );
    print_duration(
        " > Phase 2.4: Tree create         ",
        &execution_timing.duration_p24,
    );
    print_duration(
        " > Phase 3  : Suffix Array        ",
        &execution_timing.duration_p3,
    );
    if debug_mode == DebugMode::Overview || debug_mode == DebugMode::Verbose {
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
