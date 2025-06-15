use crate::files::fasta::get_fasta_content;
use crate::files::paths::get_path_in_generated_folder;
use crate::plot::plot::draw_plot_from_monitor;
use crate::suffix_array::classic_suffix_array::compute_classic_suffix_array;
use crate::suffix_array::new_suffix_array::compute_innovative_suffix_array;
use std::time::Duration;

// SUITE COMPLETE FOR CLASSIC VS INNOVATIVE COMPUTATION
pub fn full_suite(
    fasta_file_name: &str,
    chunk_size_vec: &Vec<Option<usize>>,
    max_duration_in_micros: u32,
    num_attempts: usize,
    log_execution: bool,
    log_fact: bool,
    log_trees_and_suffix_array: bool,
    draw_plot: bool,
) {
    println!("\n\nCOMPUTING SUITE ON FILE: \"{}\"\n", fasta_file_name);

    // READING FILE
    let str = &get_fasta_content(get_path_in_generated_folder(fasta_file_name));

    // SUMS FOR MEAN
    let mut sum_classic_micros = 0;
    let mut sum_innovative_micros_vec = Vec::new();
    for _ in 0..chunk_size_vec.len() {
        sum_innovative_micros_vec.push((0, 0, 0));
    }

    // MULTIPLE ATTEMPTS
    for i_attempt in 1..=num_attempts {
        println!(" > NUM ATTEMPT: {}/{}", i_attempt, num_attempts);

        // CLASSIC SUFFIX ARRAY
        let classic_suffix_array_computation = compute_classic_suffix_array(str);
        let classic_suffix_array = classic_suffix_array_computation.suffix_array;
        sum_classic_micros += classic_suffix_array_computation.duration.as_micros() as u64;

        // INNOVATIVE SUFFIX ARRAY
        let mut i = 0;
        for &chunk_size in chunk_size_vec {
            let innovative_suffix_array_computation = compute_innovative_suffix_array(
                fasta_file_name,
                str,
                chunk_size,
                log_execution,
                log_fact,
                log_trees_and_suffix_array,
            );

            // VERIFICATION
            {
                let suffix_array = innovative_suffix_array_computation.suffix_array;
                let mut success = true;
                if suffix_array.len() != classic_suffix_array.len() {
                    success = false;
                    println!("Computed Suffix Array is insufficient in size");
                } else {
                    let mut i = 0;
                    while i < classic_suffix_array.len() {
                        let clas_sa_item = classic_suffix_array[i];
                        let inn_sa_item = suffix_array[i];
                        if inn_sa_item != clas_sa_item {
                            println!("Computed Suffix Array is insufficient: element [{}] should be \"{}\" but is \"{}\"", i, clas_sa_item, inn_sa_item);
                            success = false;
                        }
                        i += 1;
                    }
                }
                if !success {
                    println!(" > Suffix Array: {:?}", suffix_array);
                    println!("Computed Suffix Array is WRONG!!! :(");
                    break;
                }
            }

            let et = &innovative_suffix_array_computation
                .execution_info
                .execution_timing;
            sum_innovative_micros_vec[i].0 += et.p1_fact.dur.as_micros() as u64;
            sum_innovative_micros_vec[i].1 += et.p2_tree.dur.as_micros() as u64;
            sum_innovative_micros_vec[i].2 += et.p3_sa.dur.as_micros() as u64;
            i += 1;
        }
    }

    // CALCULATING MEANS AND PRINTING
    println!("CLASSIC SUFFIX ARRAY CALCULATION");
    let mean_classic_micros = (sum_classic_micros as f32 / num_attempts as f32) as u64;
    print_duration(" > Sorting GSs duration   ", mean_classic_micros);
    println!("INNOVATIVE SUFFIX ARRAY CALCULATION");
    let mut chunk_size_and_phase_micros_list = Vec::new();
    let mut i = 0;
    for &chunk_size in chunk_size_vec {
        let sum_micros = &sum_innovative_micros_vec[i];
        let micros = (
            (sum_micros.0 as f32 / num_attempts as f32) as u64,
            (sum_micros.1 as f32 / num_attempts as f32) as u64,
            (sum_micros.2 as f32 / num_attempts as f32) as u64,
        );
        let chunk_size_or_zero = chunk_size.unwrap_or(0);
        println!("[CHUNK SIZE={chunk_size_or_zero}]");
        print_duration(" > Phase 1: Factorization ", micros.0);
        print_duration(" > Phase 2: Prefix Tree   ", micros.1);
        print_duration(" > Phase 3: Suffix Array  ", micros.2);
        chunk_size_and_phase_micros_list.push((chunk_size_or_zero, micros));
        i += 1;
    }

    // PLOT
    if draw_plot {
        draw_plot_from_monitor(
            fasta_file_name,
            mean_classic_micros,
            chunk_size_and_phase_micros_list,
            max_duration_in_micros,
        );
    }
}

fn print_duration(prefix: &str, micros: u64) {
    let duration = Duration::from_micros(micros);
    println!(
        "{}: {:10} micros / {:10.3} seconds",
        prefix,
        duration.as_micros(),
        duration.as_secs_f64()
    );
}
