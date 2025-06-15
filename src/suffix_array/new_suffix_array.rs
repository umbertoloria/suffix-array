use crate::factorization::icfl::get_icfl_indexes;
use crate::factorization::logging::log_factorization;
use crate::files::paths::{
    get_path_for_project_factorization_file, get_path_for_project_folder,
    get_path_for_project_full_tree_file, get_path_for_project_mini_tree_file,
    get_path_for_project_outcome_file_json, get_path_for_project_suffix_array_file,
    get_path_for_project_timing_file_json, get_path_for_project_tree_file,
};
use crate::suffix_array::chunking::get_custom_factors_and_more_using_chunk_size;
use crate::suffix_array::log_execution_info::ExecutionInfoFileFormat;
use crate::suffix_array::log_execution_outcome::ExecutionOutcomeFileFormat;
use crate::suffix_array::monitor::{ExecutionInfo, Monitor};
use crate::suffix_array::prefix_tree::logging::{log_tree, TreeLogMode};
use crate::suffix_array::prefix_tree::tree::create_tree;
use crate::suffix_array::suffix_array::suffix_array_logger::{
    log_suffix_array, make_sure_directory_exist,
};
use crate::utils::json::dump_json_in_file;

// INNOVATIVE SUFFIX ARRAY
pub struct InnovativeSuffixArrayComputationResults {
    pub suffix_array: Vec<usize>,
    pub execution_info: ExecutionInfo,
}
pub fn compute_innovative_suffix_array(
    fasta_file_name: &str,
    str: &str,
    chunk_size: Option<usize>,
    log_execution: bool,
    log_fact: bool,
    log_trees_and_suffix_array: bool,
) -> InnovativeSuffixArrayComputationResults {
    let chunk_size_or_zero = chunk_size.unwrap_or(0);
    let mut monitor = Monitor::new();
    monitor.whole_duration.start();

    // FACTORIZATION
    monitor.p1_fact.start();
    // ICFL Factorization
    let str_length = str.len();
    let str_chars = str.chars().collect::<Vec<_>>();
    let icfl_indexes = get_icfl_indexes(&str_chars);
    // Custom Factorization
    let (
        //
        factor_indexes,
        idx_to_is_custom,
        idx_to_icfl_factor,
    ) = get_custom_factors_and_more_using_chunk_size(&icfl_indexes, chunk_size, str_length);
    monitor.p1_fact.stop();
    if log_fact {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        log_factorization(
            &factor_indexes,
            &icfl_indexes,
            str,
            get_path_for_project_factorization_file(fasta_file_name, chunk_size_or_zero),
        );
    }

    // TREE
    monitor.p2_tree.start();
    let mut tree = create_tree(
        &str_chars,
        &factor_indexes,
        &icfl_indexes,
        &idx_to_is_custom,
        &mut monitor,
    );
    monitor.p2_tree.stop();
    if cfg!(feature = "verbose") {
        println!("Before SUFFIX ARRAY PHASE");
        print_for_human_like_debug(
            str,
            str_length,
            &icfl_indexes,
            &factor_indexes,
            &idx_to_icfl_factor,
            &idx_to_is_custom,
        );
        tree.print();
    }
    if log_trees_and_suffix_array {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        /*
        log_tree(
            &tree,
            TreeLogMode::Tree,
            get_path_for_project_tree_file(fasta_file_name, chunk_size_or_zero),
        );
        log_tree(
            &tree,
            TreeLogMode::FullTree,
            get_path_for_project_full_tree_file(fasta_file_name, chunk_size_or_zero),
        );
        */
        log_tree(
            &tree,
            TreeLogMode::MiniTree,
            get_path_for_project_mini_tree_file(fasta_file_name, chunk_size_or_zero),
        );
    }

    // SUFFIX ARRAY
    monitor.p3_sa.start();
    let suffix_array = tree.compute_suffix_array(
        str_length,
        str,
        &icfl_indexes,
        &idx_to_is_custom,
        &idx_to_icfl_factor,
        &mut monitor,
    );
    monitor.p3_sa.stop();
    if cfg!(feature = "verbose") {
        println!("After SUFFIX ARRAY PHASE");
        tree.print();
    }
    if log_trees_and_suffix_array {
        log_suffix_array(
            &suffix_array,
            get_path_for_project_suffix_array_file(fasta_file_name, chunk_size_or_zero),
        );
    }

    monitor.whole_duration.stop();

    let execution_info = monitor.transform_info_execution_info();
    if log_execution {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        // Execution Outcome JSON file
        let execution_outcome_file_format =
            ExecutionOutcomeFileFormat::new(&execution_info.execution_outcome);
        dump_json_in_file(
            &execution_outcome_file_format,
            get_path_for_project_outcome_file_json(fasta_file_name, chunk_size_or_zero),
        );

        // Execution Timing JSON file
        let execution_timing_file_format =
            ExecutionInfoFileFormat::new(&execution_info.execution_timing);
        dump_json_in_file(
            &execution_timing_file_format,
            get_path_for_project_timing_file_json(fasta_file_name, chunk_size_or_zero),
        );
    }

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
    factor_indexes: &Vec<usize>,
    idx_to_icfl_factor: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
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
    // IDX TO ICFL FACTOR
    for i in 0..str_length {
        print!(" {:2} ", idx_to_icfl_factor[i]);
    }
    println!("   <= IDX TO ICFL FACTOR {:?}", icfl_indexes);
    let mut i = 0;

    print_indexes_list(&icfl_indexes, str_length);
    println!("<= ICFL FACTOR INDEXES {:?}", icfl_indexes);
    print_indexes_list(&factor_indexes, str_length);
    println!("<= FACTOR INDEXES {:?}", factor_indexes);

    // IDX TO IS CUSTOM FACTOR
    i = 0;
    while i < str_length {
        print!("  {} ", if idx_to_is_custom[i] { "x" } else { " " });
        i += 1;
    }
    println!("   <= IDX TO IS CUSTOM FACTOR");
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
