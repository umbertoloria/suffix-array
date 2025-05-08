use crate::factorization::icfl::get_icfl_indexes;
use crate::files::paths::{
    get_path_for_project_folder, get_path_for_project_outcome_file_json,
    get_path_for_project_prefix_trie_file, get_path_for_project_suffix_array_file,
    get_path_for_project_timing_file_json,
};
use crate::suffix_array::chunking::get_custom_factors_and_more;
use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::log_execution_info::ExecutionInfoFileFormat;
use crate::suffix_array::log_execution_outcome::ExecutionOutcomeFileFormat;
use crate::suffix_array::monitor::{ExecutionInfo, Monitor};
use crate::suffix_array::prefix_tree::in_prefix_merge::IPMergeParams;
use crate::suffix_array::prefix_tree::new_tree::log_new_tree;
use crate::suffix_array::prefix_tree::new_tree_create::create_new_tree;
use crate::suffix_array::suffix_array::suffix_array_logger::{
    log_suffix_array, make_sure_directory_exist,
};
use crate::utils::json::dump_json_in_file;
use std::process::exit;

// INNOVATIVE SUFFIX ARRAY
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DebugMode {
    Silent,
    Overview,
    Verbose,
}
pub struct InnovativeSuffixArrayComputationResults {
    pub suffix_array: Vec<usize>,
    pub execution_info: ExecutionInfo,
}
pub fn compute_innovative_suffix_array(
    fasta_file_name: &str,
    str: &str,
    chunk_size: Option<usize>,
    perform_logging: bool,
    debug_mode: DebugMode,
) -> InnovativeSuffixArrayComputationResults {
    let mut monitor = Monitor::new();
    monitor.whole_duration.start();

    // ICFL Factorization
    monitor.p11_icfl.start();
    let str_length = str.len();
    let s_bytes = str.as_bytes();
    let icfl_indexes = get_icfl_indexes(s_bytes);
    monitor.p11_icfl.stop();

    // Custom Factorization
    monitor.p12_cust_fact.start();
    let mut custom_indexes = Vec::new();
    let mut is_custom_vec = Vec::new();
    let mut icfl_factor_list = Vec::new();
    if let Some(chunk_size) = chunk_size {
        let (custom_indexes_, is_custom_vec_, icfl_factor_list_) =
            get_custom_factors_and_more(&icfl_indexes, chunk_size, str_length);
        custom_indexes = custom_indexes_;
        is_custom_vec = is_custom_vec_;
        icfl_factor_list = icfl_factor_list_;
    } else {
        println!("Lethal with medium files on small PCs => STOP");
        exit(0x0100);
        // TODO: Disable this code since will burn your little laptop :_(
        /*
        let (custom_indexes_, is_custom_vec_, icfl_factor_list_) =
            get_icfl_factors_and_more_avoiding_custom_factorization(str_length, &icfl_indexes);
        custom_indexes = custom_indexes_;
        is_custom_vec = is_custom_vec_;
        icfl_factor_list = icfl_factor_list_;
        */
    }
    monitor.p12_cust_fact.stop();

    // Tree Create
    monitor.p21_tree_create.start();
    let mut tree = create_new_tree(
        s_bytes,
        &custom_indexes,
        &is_custom_vec,
        &mut monitor,
        debug_mode == DebugMode::Verbose,
    );
    monitor.p21_tree_create.stop();

    // +
    if debug_mode == DebugMode::Verbose {
        println!("Before merge");
        tree.print();
    }
    // -

    // +
    let chunk_size_num_for_log = chunk_size.unwrap_or(0);
    if perform_logging {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        log_new_tree(
            &tree,
            get_path_for_project_prefix_trie_file(fasta_file_name, chunk_size_num_for_log),
        );
    }

    // Re-order IDs (?)
    monitor.p22_shrink.start();
    // TODO: Remove this phase or just Order IDs in a DFS-fashion.
    monitor.p22_shrink.stop();

    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        print_for_human_like_debug(
            str,
            str_length,
            &icfl_indexes,
            &custom_indexes,
            &icfl_factor_list,
            &is_custom_vec,
        );
    }

    if debug_mode == DebugMode::Verbose {
        println!("Before IN-PREFIX MERGE");
        tree.print();
    }
    // -

    // FOR DEBUG PURPOSES
    // prefix_trie.debug_dfs();

    // In-prefix Merge
    monitor.p23_in_prefix_merge.start();
    let mut compare_cache = CompareCache::new();
    let mut ip_merge_params = IPMergeParams {
        str,
        icfl_indexes: &icfl_indexes,
        is_custom_vec: &is_custom_vec,
        icfl_factor_list: &icfl_factor_list,
        compare_cache: &mut compare_cache,
    };
    let sa = tree.in_prefix_merge(
        str_length,
        &mut ip_merge_params,
        &mut monitor,
        debug_mode == DebugMode::Verbose,
    );
    monitor.p23_in_prefix_merge.stop();

    // +
    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        println!("After IN-PREFIX MERGE");
        tree.print();
    }
    if perform_logging {
        /*log_new_tree_using_prog_sa(
            &tree,
            get_path_for_project_prefix_tree_file(fasta_file_name, chunk_size_num_for_log),
            &prog_sa,
        );*/
        // TODO: Unable to log Tree with "Inherited Rankings" since In-prefix Merge Phase eats them
        /*log_new_tree(
            &tree,
            get_path_for_project_prefix_tree_file(fasta_file_name, chunk_size_num_for_log),
        );*/
    }
    // -

    // Suffix Array
    monitor.p3_suffix_array.start();
    // let mut sa_old = Vec::new();
    // tree.prepare_get_common_prefix_partition(&mut sa_old, debug_mode == DebugMode::Verbose);
    monitor.p3_suffix_array.stop();

    // +
    if perform_logging {
        log_suffix_array(
            &sa,
            get_path_for_project_suffix_array_file(fasta_file_name, chunk_size_num_for_log),
        );
    }
    // -

    monitor.whole_duration.stop();

    // +
    let execution_info = monitor.transform_info_execution_info();
    if perform_logging {
        // Execution Outcome JSON file
        let execution_outcome_file_format =
            ExecutionOutcomeFileFormat::new(&execution_info.execution_outcome);
        dump_json_in_file(
            &execution_outcome_file_format,
            get_path_for_project_outcome_file_json(fasta_file_name, chunk_size_num_for_log),
        );

        // Execution Timing JSON file
        let execution_timing_file_format =
            ExecutionInfoFileFormat::new(&execution_info.execution_timing);
        dump_json_in_file(
            &execution_timing_file_format,
            get_path_for_project_timing_file_json(fasta_file_name, chunk_size_num_for_log),
        );
    }
    // -

    // println!("Total time: {}", duration.as_secs_f32());

    InnovativeSuffixArrayComputationResults {
        suffix_array: sa,
        execution_info,
    }
}
fn print_for_human_like_debug(
    str: &str,
    str_length: usize,
    icfl_indexes: &Vec<usize>,
    custom_indexes: &Vec<usize>,
    icfl_factor_list: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    // depths: &Vec<usize>,
) {
    // CHAR INDEXES
    for i in 0..str_length {
        print!(" {:2} ", i);
    }
    println!();
    // CHARS
    for i in 0..str_length {
        print!("  {} ", &str[i..i + 1]);
    }
    println!();
    // ICFL FACTORS
    for i in 0..str_length {
        print!(" {:2} ", icfl_factor_list[i]);
    }
    println!("   <= ICFL FACTORS {:?}", icfl_indexes);
    let mut i = 0;

    print_indexes_list(&icfl_indexes, str_length);
    println!("<= ICFL FACTORS {:?}", icfl_indexes);
    print_indexes_list(&custom_indexes, str_length);
    println!("<= CUSTOM FACTORS {:?}", custom_indexes);

    i = 0;
    while i < str_length {
        print!("  {} ", if is_custom_vec[i] { "x" } else { " " });
        i += 1;
    }
    println!("   <= IS IN CUSTOM FACTOR");
    /*for i in 0..str_length {
        print!(" {:2} ", depths[i]);
    }
    println!("   <= DEPTHS");*/
}
fn print_indexes_list(indexes_list: &Vec<usize>, str_length: usize) {
    let mut iter = &mut indexes_list.iter();
    iter.next(); // Skipping the first because it's always "0".
    let mut last = 0;
    print!("|");
    while let Some(&custom_factor_index) = iter.next() {
        print!("{}|", " ".repeat((custom_factor_index - last) * 4 - 1));
        last = custom_factor_index;
    }
    print!("{}|  ", " ".repeat((str_length - last) * 4 - 1));
}
