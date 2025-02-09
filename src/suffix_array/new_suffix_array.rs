use crate::factorization::icfl::icfl;
use crate::files::paths::{
    get_path_for_project_folder, get_path_for_project_monitor_file,
    get_path_for_project_prefix_tree_file, get_path_for_project_prefix_trie_file,
    get_path_for_project_suffix_array_file,
};
use crate::suffix_array::chunking::{get_custom_factors_and_more, get_indexes_from_factors};
use crate::suffix_array::monitor::{
    log_monitor_after_process_ended, log_prefix_trie, ExecutionInfo, Monitor,
};
use crate::suffix_array::prefix_tree::{
    create_prefix_tree_from_prefix_trie, log_prefix_tree, log_suffix_array,
    make_sure_directory_exist,
};
use crate::suffix_array::prefix_trie::create_prefix_trie;
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
    let src_length = str.len();

    let mut monitor = Monitor::new();
    monitor.process_start();

    // ICFL Factorization
    monitor.phase1_1_icfl_factorization_start();
    let factors = icfl(str);
    let icfl_indexes = get_indexes_from_factors(&factors);

    // Custom Factorization
    monitor.phase1_2_custom_factorization_start();
    let mut custom_indexes = Vec::new();
    let mut is_custom_vec = Vec::new();
    let mut icfl_factor_list = Vec::new();
    if let Some(chunk_size) = chunk_size {
        let (custom_indexes_, is_custom_vec_, icfl_factor_list_) =
            get_custom_factors_and_more(&icfl_indexes, chunk_size, src_length);
        custom_indexes = custom_indexes_;
        is_custom_vec = is_custom_vec_;
        icfl_factor_list = icfl_factor_list_;
    } else {
        println!("Lethal with medium files on small PCs => STOP");
        exit(0x0100);
        // TODO: Disable this code since will burn your little laptop :_(
        /*
        let (custom_indexes_, is_custom_vec_, icfl_factor_list_) =
            get_icfl_factors_and_more_avoiding_custom_factorization(src_length, &icfl_indexes);
        custom_indexes = custom_indexes_;
        is_custom_vec = is_custom_vec_;
        icfl_factor_list = icfl_factor_list_;
        */
    }

    // Prefix Trie Structure create
    monitor.phase2_1_prefix_trie_create_start();
    let mut prefix_trie = create_prefix_trie(
        str,
        src_length,
        &custom_indexes,
        &is_custom_vec,
        &mut monitor,
    );
    monitor.phase2_1_prefix_trie_create_stop();

    // +
    if debug_mode == DebugMode::Verbose {
        println!("Before merge");
        prefix_trie.print(0, "".into());
    }
    // -

    // Merge Rankings (Canonical and Custom)
    monitor.phase2_2_prefix_trie_merge_rankings_start();
    let mut wbsa = (0..src_length).collect::<Vec<_>>();
    let mut depths = vec![0usize; src_length];
    prefix_trie.merge_rankings_and_sort_recursive(str, &mut wbsa, &mut depths, 0);
    monitor.phase2_2_prefix_trie_merge_rankings_stop();

    // +
    let chunk_size_num_for_log = if let Some(chunk_size) = chunk_size {
        chunk_size
    } else {
        0
    };
    if perform_logging {
        make_sure_directory_exist(get_path_for_project_folder(fasta_file_name));
        log_prefix_trie(
            &prefix_trie,
            &wbsa,
            get_path_for_project_prefix_trie_file(fasta_file_name, chunk_size_num_for_log),
        );
    }

    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        print_for_human_like_debug(
            str,
            src_length,
            &icfl_indexes,
            &custom_indexes,
            &icfl_factor_list,
            &is_custom_vec,
            &depths,
        );
    }

    if debug_mode == DebugMode::Verbose {
        println!("Before SHRINK");
        prefix_trie.print_with_wbsa(0, "".into(), &wbsa);
    }
    // -

    monitor.phase2_3_prefix_trie_in_prefix_merge_start();
    prefix_trie.in_prefix_merge(
        str,
        &mut wbsa,
        &mut depths,
        &icfl_indexes,
        &is_custom_vec,
        &icfl_factor_list,
        &mut monitor,
        debug_mode == DebugMode::Verbose,
    );
    monitor.phase2_3_prefix_trie_in_prefix_merge_stop();

    /*
    prefix_trie.shrink_bottom_up(
        &mut wbsa,
        &mut depths,
        str,
        &icfl_indexes,
        &is_custom_vec,
        &icfl_factor_list,
    );
    match debug_mode {
        DebugMode::Overview => {
            println!("After SHRINK");
            prefix_trie.print_with_wbsa(0, "".into(), &wbsa);
            println!("{:?}", wbsa);
        }
        _ => {}
    }
    */

    // +
    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        println!("After IN_PREFIX_MERGE");
        prefix_trie.print_with_wbsa(0, "".into(), &wbsa);
    }
    // -

    monitor.phase2_4_prefix_tree_create_start();
    let mut prefix_tree = create_prefix_tree_from_prefix_trie(prefix_trie, &mut wbsa);
    monitor.phase2_4_prefix_tree_create_stop();

    // +
    if debug_mode == DebugMode::Verbose || debug_mode == DebugMode::Overview {
        prefix_tree.print(str);
    }
    if perform_logging {
        log_prefix_tree(
            &prefix_tree,
            str,
            get_path_for_project_prefix_tree_file(fasta_file_name, chunk_size_num_for_log),
        );
    }
    // -

    monitor.phase3_suffix_array_compose_start();
    let mut sa = Vec::new();
    // prefix_trie.dump_onto_wbsa(&mut wbsa, &mut sa, 0);
    prefix_tree.prepare_get_common_prefix_partition(&mut sa, str, debug_mode == DebugMode::Verbose);
    monitor.phase3_suffix_array_compose_stop();

    // +
    if perform_logging {
        log_suffix_array(
            &sa,
            get_path_for_project_suffix_array_file(fasta_file_name, chunk_size_num_for_log),
        );
    }
    // -

    monitor.process_end();

    // +
    let execution_info = monitor.transform_info_execution_info();
    if perform_logging {
        log_monitor_after_process_ended(
            &execution_info.0,
            get_path_for_project_monitor_file(fasta_file_name, chunk_size_num_for_log),
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
    src: &str,
    src_length: usize,
    icfl_indexes: &Vec<usize>,
    custom_indexes: &Vec<usize>,
    icfl_factor_list: &Vec<usize>,
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
        print!(" {:2} ", icfl_factor_list[i]);
    }
    println!("   <= ICFL FACTORS {:?}", icfl_indexes);
    let mut i = 0;

    print_indexes_list(&icfl_indexes, src_length);
    println!("<= ICFL FACTORS {:?}", icfl_indexes);
    print_indexes_list(&custom_indexes, src_length);
    println!("<= CUSTOM FACTORS {:?}", custom_indexes);

    i = 0;
    while i < src_length {
        print!("  {} ", if is_custom_vec[i] { "x" } else { " " });
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
