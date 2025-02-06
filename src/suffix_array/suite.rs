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
    let src = get_fasta_content(get_path_in_generated_folder(fasta_file_name));
    let src_str = src.as_str();

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
    let mut chunk_data = HashMap::new();
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
            &mut chunk_data,
            Some(chunk_size),
        );
        if test_result_ok {
            break;
        }
    }

    // Plots
    let mut data = Vec::new();
    for chunk_size in chunks_interval {
        let (duration, monitor) = chunk_data.remove(&chunk_size).unwrap();
        data.push((chunk_size, duration, monitor));
    }
    draw_plot_from_monitor(fasta_file_name, data);
}

fn run_and_validate_test(
    fasta_file_name: &str,
    debug_mode: DebugMode,
    src_str: &str,
    classic_suffix_array: &Vec<usize>,
    chunk_data: &mut HashMap<usize, (Duration, Monitor)>,
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
    let duration = monitor.get_process_duration().unwrap();
    println!(" > Duration: {:15} micros", duration.as_micros());
    println!(" > Duration: {:15.3} seconds", duration.as_secs_f64());
    if debug_mode == DebugMode::Overview || debug_mode == DebugMode::Verbose {
        monitor.print();
    }
    // println!(" > Suffix Array: {:?}", wbsa);
    chunk_data.insert(chunk_size_or_zero, (duration, monitor));

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
