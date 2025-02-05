use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use crate::files::paths::{
    get_path_for_project_folder, get_path_for_project_prefix_tree_file,
    get_path_for_project_prefix_trie_file, get_path_for_project_suffix_array_file,
    get_path_in_generated_folder,
};
use crate::plot::plot::draw_histogram_from_prefix_trie_monitor;
use crate::suffix_array::chunking::{get_custom_factors_and_more, get_indexes_from_factors};
use crate::suffix_array::prefix_tree::{
    create_prefix_tree_from_prefix_trie, log_prefix_tree, log_suffix_array,
    make_sure_directory_exist,
};
use crate::suffix_array::prefix_trie::{create_prefix_trie, log_prefix_trie, PrefixTrieMonitor};
use crate::suffix_array::sorter::sort_pair_vector_of_indexed_strings;
use std::collections::HashMap;
use std::time::{Duration, Instant};

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
    // println!("STRING={}", src_str);

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
    for &chunk_size in &chunks_interval {
        let innovative_suffix_array_computation =
            compute_innovative_suffix_array(fasta_file_name, src_str, Some(chunk_size), debug_mode);
        let wbsa = innovative_suffix_array_computation.suffix_array;
        let prefix_trie_monitor = innovative_suffix_array_computation.prefix_trie_monitor;
        println!("[CHUNK SIZE={chunk_size}]");
        println!(
            " > Duration: {:15} micros",
            innovative_suffix_array_computation.duration.as_micros()
        );
        println!(
            " > Duration: {:15.3} seconds",
            innovative_suffix_array_computation.duration.as_secs_f64()
        );
        if debug_mode == DebugMode::Overview || debug_mode == DebugMode::Verbose {
            prefix_trie_monitor.print();
        }
        // println!(" > Suffix Array: {:?}", wbsa);
        chunk_data.insert(
            chunk_size,
            (
                // Duration
                innovative_suffix_array_computation.duration,
                // Monitor
                prefix_trie_monitor,
            ),
        );

        // VERIFICATION
        let mut success = true;
        if wbsa.len() != classic_suffix_array.len() {
            success = false;
            println!("Wanna Be Suffix Array is insufficient in size");
        } else {
            let mut i = 0;
            while i < classic_suffix_array.len() {
                let sa_item = classic_suffix_array[i];
                let wbsa_item = wbsa[i];
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
            println!(" > Suffix Array: {:?}", wbsa);
            println!("Wanna Be Suffix Array is WRONG!!! :(");

            break;
        }
    }
    let mut data = Vec::new();
    for chunk_size in chunks_interval {
        let (duration, prefix_trie_monitor) = chunk_data.remove(&chunk_size).unwrap();
        data.push((chunk_size, duration, prefix_trie_monitor));
    }
    draw_histogram_from_prefix_trie_monitor(fasta_file_name, data);
}

// CLASSIC SUFFIX ARRAY
struct ClassicSuffixArrayComputationResults<'a> {
    suffix_array: Vec<usize>,
    suffix_array_pairs: Vec<(usize, &'a str)>,
    duration: Duration,
}
fn compute_classic_suffix_array(
    src: &str,
    debug_verbose: bool,
) -> ClassicSuffixArrayComputationResults {
    let before = Instant::now();

    let mut suffix_array_pairs = Vec::new();
    // Create array of global suffixes
    for i in 0..src.len() {
        suffix_array_pairs.push((i, &src[i..]));
    }
    // Create sort by comparing global suffixes
    sort_pair_vector_of_indexed_strings(&mut suffix_array_pairs);
    // Extracting only indexes of previous array of pairs
    let mut suffix_array_indexes = Vec::new();
    for &(index, _) in &suffix_array_pairs {
        suffix_array_indexes.push(index);
    }
    let after = Instant::now();

    if debug_verbose {
        for (index, suffix) in &suffix_array_pairs {
            println!(" > SUFFIX_ARRAY [{:3}] = {}", index, suffix);
        }
    }

    // println!("Total time: {}", duration.as_secs_f32());

    ClassicSuffixArrayComputationResults {
        suffix_array: suffix_array_indexes,
        suffix_array_pairs,
        duration: after - before,
    }
}

// INNOVATIVE SUFFIX ARRAY
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DebugMode {
    Silent,
    Overview,
    Verbose,
}
struct InnovativeSuffixArrayComputationResults {
    suffix_array: Vec<usize>,
    prefix_trie_monitor: PrefixTrieMonitor,
    duration: Duration,
}
fn compute_innovative_suffix_array(
    fasta_file_name: &str,
    str: &str,
    chunk_size: Option<usize>,
    debug_mode: DebugMode,
) -> InnovativeSuffixArrayComputationResults {
    let before = Instant::now();

    let src_length = str.len();

    // ICFL Factorization
    let factors = icfl(str);
    // TODO: Simplify algorithms by having string length as last item of these Factor Index vectors
    let icfl_indexes = get_indexes_from_factors(&factors);

    // Custom Factorization
    let mut custom_indexes = Vec::new();
    let mut is_custom_vec = Vec::new();
    let mut factor_list = Vec::new();
    if let Some(chunk_size) = chunk_size {
        let (
            //
            custom_indexes_,
            is_custom_vec_,
            factor_list_,
        ) = get_custom_factors_and_more(&icfl_indexes, chunk_size, src_length);
        custom_indexes = custom_indexes_;
        is_custom_vec = is_custom_vec_;
        factor_list = factor_list_;
        // let custom_factors = get_custom_factor_strings_from_custom_indexes(str, &custom_indexes);
        // println!("{:?}", custom_factors);
    } else {
        // TODO: Test this
        for i in 0..icfl_indexes.len() {
            let cur_factor_index = icfl_indexes[i];

            // Curr Factor Size
            let next_factor_index = if i < icfl_indexes.len() - 1 {
                icfl_indexes[i + 1]
            } else {
                src_length
            };
            let cur_factor_size = next_factor_index - cur_factor_index;

            // Updating "custom_indexes"
            custom_indexes.push(cur_factor_index);

            // Updating "is_custom_vec"
            // Updating "factor_list"
            for _ in 0..cur_factor_size {
                is_custom_vec.push(false);
                factor_list.push(i);
            }
        }
    }

    let chunk_size_num_for_log = if let Some(chunk_size) = chunk_size {
        chunk_size
    } else {
        0
    };

    // Prefix Trie Structure create
    let mut prefix_trie = create_prefix_trie(str, src_length, &custom_indexes, &is_custom_vec);
    let mut prefix_trie_monitor = PrefixTrieMonitor::new();

    if debug_mode == DebugMode::Verbose {
        println!("Before merge");
        prefix_trie.print(0, "".into());
    }

    // Merge Rankings (Canonical and Custom)
    let mut wbsa = (0..src_length).collect::<Vec<_>>();
    let mut depths = vec![0usize; src_length];
    prefix_trie.merge_rankings_and_sort_recursive(str, &mut wbsa, &mut depths, 0);
    make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
    log_prefix_trie(
        &prefix_trie,
        &wbsa,
        get_path_for_project_prefix_trie_file(fasta_file_name, chunk_size_num_for_log),
    );

    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        print_for_human_like_debug(
            str,
            src_length,
            &icfl_indexes,
            &custom_indexes,
            &factor_list,
            &is_custom_vec,
            &depths,
        );
    }

    if debug_mode == DebugMode::Verbose {
        println!("Before SHRINK");
        prefix_trie.print_with_wbsa(0, "".into(), &wbsa);
    }

    prefix_trie.in_prefix_merge(
        str,
        &mut wbsa,
        &mut depths,
        &icfl_indexes,
        &is_custom_vec,
        &factor_list,
        &mut prefix_trie_monitor,
        debug_mode == DebugMode::Verbose,
    );

    /*prefix_trie.shrink_bottom_up(
        &mut wbsa,
        &mut depths,
        str,
        &icfl_indexes,
        &is_custom_vec,
        &factor_list,
    );
    match debug_mode {
        DebugMode::Overview => {
            println!("After SHRINK");
            prefix_trie.print_with_wbsa(0, "".into(), &wbsa);
            println!("{:?}", wbsa);
        }
        _ => {}
    }*/

    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        println!("After IN_PREFIX_MERGE");
        prefix_trie.print_with_wbsa(0, "".into(), &wbsa);
    }

    let mut prefix_tree = create_prefix_tree_from_prefix_trie(prefix_trie, &mut wbsa);
    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        prefix_tree.print();
    }
    log_prefix_tree(
        &prefix_tree,
        get_path_for_project_prefix_tree_file(fasta_file_name, chunk_size_num_for_log),
    );

    let mut sa = Vec::new();
    // prefix_trie.dump_onto_wbsa(&mut wbsa, &mut sa, 0);
    prefix_tree.prepare_get_common_prefix_partition(&mut sa, debug_mode == DebugMode::Verbose);

    log_suffix_array(
        &sa,
        get_path_for_project_suffix_array_file(fasta_file_name, chunk_size_num_for_log),
    );

    let after = Instant::now();

    // println!("Total time: {}", duration.as_secs_f32());

    InnovativeSuffixArrayComputationResults {
        suffix_array: sa,
        prefix_trie_monitor,
        duration: after - before,
    }
}
fn print_for_human_like_debug(
    src: &str,
    src_length: usize,
    icfl_indexes: &Vec<usize>,
    custom_indexes: &Vec<usize>,
    factor_list: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    depths: &Vec<usize>,
) {
    // CHAR INDEXES
    for i in 0..src_length {
        print!(" {:2} ", i);
    }
    println!();
    // CHARS
    for i in 0..src_length {
        print!("  {} ", &src[i..i + 1]);
    }
    println!();
    // ICFL FACTORS
    for i in 0..src_length {
        print!(" {:2} ", factor_list[i]);
    }
    println!("   <= ICFL FACTORS {:?}", icfl_indexes);
    let mut i = 0;

    print_indexes_list(&icfl_indexes, src_length);
    println!("<= ICFL FACTORS {:?}", icfl_indexes);
    print_indexes_list(&custom_indexes, src_length);
    println!("<= CUSTOM FACTORS {:?}", custom_indexes);

    i = 0;
    while i < src_length {
        print!("  {} ", if is_custom_vec[i] { "1" } else { " " });
        i += 1;
    }
    println!("   <= IS IN CUSTOM FACTOR");
    for i in 0..src_length {
        print!(" {:2} ", depths[i]);
    }
    println!("   <= DEPTHS");
}
fn print_indexes_list(indexes_list: &Vec<usize>, src_length: usize) {
    let mut iter = &mut indexes_list.iter();
    iter.next(); // Skipping the first because it's always "0".
    let mut last = 0;
    print!("|");
    while let Some(&custom_factor_index) = iter.next() {
        print!("{}|", " ".repeat((custom_factor_index - last) * 4 - 1));
        last = custom_factor_index;
    }
    print!("{}|  ", " ".repeat((src_length - last) * 4 - 1));
}
