use crate::files::fasta::get_fasta_content;
use crate::files::paths::get_path_in_generated_folder;
use crate::plot::plot::draw_plot_from_monitor;
use crate::suffix_array::classic_suffix_array::compute_classic_suffix_array;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::new_suffix_array::{compute_innovative_suffix_array, DebugMode};
use std::collections::HashMap;
use std::time::Duration;

// SUITE COMPLETE FOR CLASSIC VS INNOVATIVE COMPUTATION
pub fn suite_complete_on_fasta_file(
    fasta_file_name: &str,
    chunk_size_interval: (usize, usize), // Both incl.
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
    let mut chunk_to_monitor = HashMap::new();
    /*
    // TODO: Lethal with medium files on small PCs
    run_and_validate_test(
        fasta_file_name,
        debug_mode,
        src_str,
        &classic_suffix_array,
        &mut chunk_data,
        None,
    );
    */
    for &chunk_size in &chunks_interval {
        let test_result_ok = run_and_validate_test(
            fasta_file_name,
            debug_mode,
            src_str,
            &classic_suffix_array,
            &mut chunk_to_monitor,
            Some(chunk_size),
        );
        if test_result_ok {
            break;
        }
    }

    // Plots
    let mut chunk_and_monitor_pairs = Vec::new();
    for chunk_size in chunks_interval {
        let monitor = chunk_to_monitor.remove(&chunk_size).unwrap();
        chunk_and_monitor_pairs.push((chunk_size, monitor));
    }
    draw_plot_from_monitor(fasta_file_name, chunk_and_monitor_pairs);
}

fn run_and_validate_test(
    fasta_file_name: &str,
    debug_mode: DebugMode,
    src_str: &str,
    classic_suffix_array: &Vec<usize>,
    chunk_to_monitor: &mut HashMap<usize, Monitor>,
    chunk_size: Option<usize>,
) -> bool {
    let innovative_suffix_array_computation =
        compute_innovative_suffix_array(fasta_file_name, src_str, chunk_size, debug_mode);
    let suffix_array = innovative_suffix_array_computation.suffix_array;
    let monitor = innovative_suffix_array_computation.monitor;

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
        &monitor.get_sum_phases_duration(),
    );
    print_duration(
        " > Duration (with extra)          ",
        &monitor.get_whole_process_duration_included_extra(),
    );
    print_duration(
        " > Phase 1.1: Factorization ICFL  ",
        &monitor.get_phase1_1_icfl_factorization_duration(),
    );
    print_duration(
        " > Phase 1.2: Factorization Custom",
        &monitor.get_phase1_2_custom_factorization_duration(),
    );
    print_duration(
        " > Phase 2.1: Trie Create         ",
        &monitor.get_phase2_1_prefix_trie_create_duration(),
    );
    print_duration(
        " > Phase 2.2: Trie Merge rankings ",
        &monitor.get_phase2_2_prefix_trie_merge_rankings_duration(),
    );
    print_duration(
        " > Phase 2.3: Trie In-prefix merge",
        &monitor.get_phase2_3_prefix_trie_in_prefix_merge_duration(),
    );
    print_duration(
        " > Phase 2.4: Tree create         ",
        &monitor.get_phase2_4_prefix_tree_create_duration(),
    );
    print_duration(
        " > Phase 3  : Suffix Array        ",
        &monitor.get_phase3_suffix_array_compose_duration(),
    );
    if debug_mode == DebugMode::Overview || debug_mode == DebugMode::Verbose {
        monitor.print();
    }
    // println!(" > Suffix Array: {:?}", wbsa);
    chunk_to_monitor.insert(chunk_size_or_zero, monitor);

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
    if success {
        // println!("Wanna Be Suffix Array is PERFECT :)");
    } else {
        println!(" > Suffix Array: {:?}", suffix_array);
        println!("Wanna Be Suffix Array is WRONG!!! :(");

        return true;
    }
    false
}

fn print_duration(prefix: &str, duration: &Duration) {
    println!(
        "{}: {:15} micros / {:15.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64()
    );
}
