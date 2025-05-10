use crate::factorization::icfl::get_icfl_indexes;
use crate::files::paths::{
    get_path_for_project_folder, get_path_for_project_outcome_file_json,
    get_path_for_project_suffix_array_file, get_path_for_project_timing_file_json,
    get_path_for_project_tree_file,
};
use crate::suffix_array::chunking::get_custom_factors_and_more;
use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::log_execution_info::ExecutionInfoFileFormat;
use crate::suffix_array::log_execution_outcome::ExecutionOutcomeFileFormat;
use crate::suffix_array::monitor::{ExecutionInfo, Monitor};
use crate::suffix_array::prefix_tree::in_prefix_merge::IPMergeParams;
use crate::suffix_array::prefix_tree::tree::{create_tree, log_tree};
use crate::suffix_array::suffix_array::suffix_array_logger::{
    log_suffix_array, make_sure_directory_exist,
};
use crate::utils::json::dump_json_in_file;
use std::process::exit;

// INNOVATIVE SUFFIX ARRAY
pub struct InnovativeSuffixArrayComputationResults {
    pub suffix_array: Vec<usize>,
    pub execution_info: ExecutionInfo,
}
pub fn compute_innovative_suffix_array(
    fasta_file_name: &str,
    str: &str,
    chunk_size: usize,
    log_execution: bool,
    log_trees_and_suffix_array: bool,
) -> InnovativeSuffixArrayComputationResults {
    let mut monitor = Monitor::new();
    monitor.whole_duration.start();

    // FACTORIZATION
    monitor.p1_fact.start();
    // ICFL Factorization
    let str_length = str.len();
    let s_bytes = str.as_bytes();
    let icfl_indexes = get_icfl_indexes(s_bytes);
    // Custom Factorization
    let mut custom_indexes = Vec::new();
    let mut is_custom_vec = Vec::new();
    let mut icfl_factor_list = Vec::new();
    let (custom_indexes_, is_custom_vec_, icfl_factor_list_) =
        get_custom_factors_and_more(&icfl_indexes, chunk_size, str_length);
    custom_indexes = custom_indexes_;
    is_custom_vec = is_custom_vec_;
    icfl_factor_list = icfl_factor_list_;
    monitor.p1_fact.stop();

    // TREE
    monitor.p2_tree.start();
    let mut tree = create_tree(s_bytes, &custom_indexes, &is_custom_vec, &mut monitor);
    monitor.p2_tree.stop();

    // +
    if cfg!(feature = "verbose") {
        println!("Before merge");
        tree.print();
    }
    // -

    // +
    if log_trees_and_suffix_array {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        log_tree(
            &tree,
            get_path_for_project_tree_file(fasta_file_name, chunk_size),
        );
    }

    if cfg!(feature = "verbose") {
        print_for_human_like_debug(
            str,
            str_length,
            &icfl_indexes,
            &custom_indexes,
            &icfl_factor_list,
            &is_custom_vec,
        );
        println!("Before IN-PREFIX MERGE");
        tree.print();
    }
    // -

    // FOR DEBUG PURPOSES
    // prefix_trie.debug_dfs();

    // SUFFIX ARRAY
    monitor.p3_sa.start();
    let mut compare_cache = CompareCache::new();
    let mut ip_merge_params = IPMergeParams {
        str,
        icfl_indexes: &icfl_indexes,
        is_custom_vec: &is_custom_vec,
        icfl_factor_list: &icfl_factor_list,
        compare_cache: &mut compare_cache,
    };
    let suffix_array = tree.in_prefix_merge_and_common_prefix_partition(
        str_length,
        &mut ip_merge_params,
        &mut monitor,
    );
    monitor.p3_sa.stop();

    // +
    if cfg!(feature = "verbose") {
        println!("After IN-PREFIX MERGE");
        tree.print();
    }
    if log_trees_and_suffix_array {
        /*log_tree_using_prog_sa(
            &tree,
            get_path_for_project_prefix_tree_file(fasta_file_name, chunk_size_num_for_log),
            &prog_sa,
        );*/
        // TODO: Unable to log Tree with "Inherited Rankings" since In-prefix Merge Phase eats them
        /*log_tree(
            &tree,
            get_path_for_project_prefix_tree_file(fasta_file_name, chunk_size_num_for_log),
        );*/
    }
    // -

    // +
    if log_trees_and_suffix_array {
        log_suffix_array(
            &suffix_array,
            get_path_for_project_suffix_array_file(fasta_file_name, chunk_size),
        );
    }
    // -

    monitor.whole_duration.stop();

    // +
    let execution_info = monitor.transform_info_execution_info();
    if log_execution {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        // Execution Outcome JSON file
        let execution_outcome_file_format =
            ExecutionOutcomeFileFormat::new(&execution_info.execution_outcome);
        dump_json_in_file(
            &execution_outcome_file_format,
            get_path_for_project_outcome_file_json(fasta_file_name, chunk_size),
        );

        // Execution Timing JSON file
        let execution_timing_file_format =
            ExecutionInfoFileFormat::new(&execution_info.execution_timing);
        dump_json_in_file(
            &execution_timing_file_format,
            get_path_for_project_timing_file_json(fasta_file_name, chunk_size),
        );
    }
    // -

    // println!("Total time: {}", duration.as_secs_f32());

    InnovativeSuffixArrayComputationResults {
        suffix_array,
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
