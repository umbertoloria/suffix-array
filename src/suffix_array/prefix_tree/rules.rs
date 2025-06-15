use crate::suffix_array::monitor::Monitor;

pub fn rules_safe(
    parent_ls_index: usize,
    child_ls_index: usize,
    child_ls_size: usize,
    str: &str,
    icfl_indexes: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
    idx_to_icfl_factor: &Vec<usize>,
    monitor: &mut Monitor,
    slow_check: bool,
) -> bool {
    if !slow_check {
        rules(
            parent_ls_index,
            child_ls_index,
            child_ls_size,
            str,
            icfl_indexes,
            idx_to_is_custom,
            idx_to_icfl_factor,
            monitor,
        )
    } else {
        let parent_ls = &str[parent_ls_index + child_ls_size..];
        let child_ls = &str[child_ls_index + child_ls_size..];
        let mut oracle = if parent_ls < child_ls {
            false // Father first.
        } else {
            true // Child first.
        };
        let given = rules(
            parent_ls_index,
            child_ls_index,
            child_ls_size,
            str,
            icfl_indexes,
            idx_to_is_custom,
            idx_to_icfl_factor,
            monitor,
        );
        if given != oracle {
            println!(
                " RULES: x={parent_ls_index:2}, y={child_ls_index:2}, offset={child_ls_size} => {oracle}, BUT GIVEN WRONG!"
            );
        }
        /*else {
            println!(" RULES: x={x:2}, y={y:2}, offset={child_ls_size} => {oracle}");
        }*/
        oracle
    }
}
fn rules(
    parent_ls_index: usize,
    child_ls_index: usize,
    child_ls_size: usize,
    str: &str,
    icfl_indexes: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
    idx_to_icfl_factor: &Vec<usize>,
    monitor: &mut Monitor,
) -> bool {
    // Return values:
    //  FALSE => GS Parent < GS Child;
    //  TRUE  => GS Child < GS Parent.
    if idx_to_is_custom[parent_ls_index] && idx_to_is_custom[child_ls_index] {
        monitor.new_compare_of_two_ls_in_custom_factors();
        monitor.new_compare_using_actual_string_compare();
        return perform_gs_comparison_a_before_b(
            str,
            child_ls_index + child_ls_size,
            parent_ls_index + child_ls_size,
        );
    }

    let last_icfl_index = icfl_indexes[icfl_indexes.len() - 1];

    if idx_to_is_custom[parent_ls_index] {
        monitor.new_compare_one_ls_in_custom_factor();
        return if idx_to_icfl_factor[parent_ls_index] <= idx_to_icfl_factor[child_ls_index] {
            monitor.new_compare_using_rules();
            if child_ls_index >= last_icfl_index {
                true
            } else {
                false
            }
        } else {
            monitor.new_compare_using_actual_string_compare();
            perform_gs_comparison_a_before_b(
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
        };
    }

    if idx_to_is_custom[child_ls_index] {
        monitor.new_compare_one_ls_in_custom_factor();
        return if idx_to_icfl_factor[child_ls_index] <= idx_to_icfl_factor[parent_ls_index] {
            monitor.new_compare_using_rules();
            if parent_ls_index >= last_icfl_index {
                false
            } else {
                true
            }
        } else {
            monitor.new_compare_using_actual_string_compare();
            perform_gs_comparison_a_before_b(
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
        };
    }

    if parent_ls_index >= last_icfl_index && child_ls_index >= last_icfl_index {
        monitor.new_compare_using_rules();
        false
    } else if idx_to_icfl_factor[parent_ls_index] == idx_to_icfl_factor[child_ls_index] {
        monitor.new_compare_using_rules();
        true
    } else {
        if parent_ls_index >= last_icfl_index {
            monitor.new_compare_using_rules();
            false
        } else if child_ls_index >= last_icfl_index {
            monitor.new_compare_using_actual_string_compare();
            perform_gs_comparison_a_before_b(
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
        } else {
            if parent_ls_index > child_ls_index {
                monitor.new_compare_using_rules();
                true
            } else {
                monitor.new_compare_using_actual_string_compare();
                perform_gs_comparison_a_before_b(
                    str,
                    child_ls_index + child_ls_size,
                    parent_ls_index + child_ls_size,
                )
            }
        }
    }
}

pub fn perform_gs_comparison_a_before_b(str: &str, ls_index_1: usize, ls_index_2: usize) -> bool {
    // println!(" -> *** comparing {} with {}", ls_index_1, ls_index_2);
    let cmp1 = &str[ls_index_1..];
    let cmp2 = &str[ls_index_2..];
    if cmp1 < cmp2 {
        true
    } else {
        false
    }
}
