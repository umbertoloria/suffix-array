use crate::prefix_tree::monitor::Monitor;

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
            false // Parent first.
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
        // + Extra
        monitor.new_compare_of_two_ls_in_custom_factors();
        monitor.new_compare_using_actual_string_compare();
        // - Extra
        return perform_gs_comparison_a_before_b(
            str,
            child_ls_index + child_ls_size,
            parent_ls_index + child_ls_size,
        );
    }

    let last_icfl_index = icfl_indexes[icfl_indexes.len() - 1];

    if idx_to_is_custom[parent_ls_index] {
        // + Extra
        monitor.new_compare_one_ls_in_custom_factor();
        // - Extra
        return if idx_to_icfl_factor[parent_ls_index] <= idx_to_icfl_factor[child_ls_index] {
            // + Extra
            monitor.new_compare_using_rules();
            // - Extra
            if child_ls_index >= last_icfl_index {
                true
            } else {
                false
            }
        } else {
            // + Extra
            monitor.new_compare_using_actual_string_compare();
            // - Extra
            perform_gs_comparison_a_before_b(
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
        };
    }

    if idx_to_is_custom[child_ls_index] {
        // + Extra
        monitor.new_compare_one_ls_in_custom_factor();
        // - Extra
        return if idx_to_icfl_factor[child_ls_index] <= idx_to_icfl_factor[parent_ls_index] {
            // + Extra
            monitor.new_compare_using_rules();
            // - Extra
            if parent_ls_index >= last_icfl_index {
                false
            } else {
                true
            }
        } else {
            // + Extra
            monitor.new_compare_using_actual_string_compare();
            // - Extra
            perform_gs_comparison_a_before_b(
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
        };
    }

    if parent_ls_index >= last_icfl_index && child_ls_index >= last_icfl_index {
        // + Extra
        monitor.new_compare_using_rules();
        // - Extra
        false
    } else if idx_to_icfl_factor[parent_ls_index] == idx_to_icfl_factor[child_ls_index] {
        // + Extra
        monitor.new_compare_using_rules();
        // - Extra
        true
    } else {
        if parent_ls_index >= last_icfl_index {
            // + Extra
            monitor.new_compare_using_rules();
            // - Extra
            false
        } else if child_ls_index >= last_icfl_index {
            // + Extra
            monitor.new_compare_using_actual_string_compare();
            // - Extra
            perform_gs_comparison_a_before_b(
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
        } else {
            if parent_ls_index > child_ls_index {
                // + Extra
                monitor.new_compare_using_rules();
                // - Extra
                true
            } else {
                // + Extra
                monitor.new_compare_using_actual_string_compare();
                // - Extra
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

pub fn compatibility_property_icfl_a_before_b(
    ls1_idx: usize,
    ls2_idx: usize,
    str: &[char],
    icfl_indexes: &Vec<usize>,
    idx_to_icfl_factor: &Vec<usize>,
) -> bool {
    let ls1_icfl_factor_idx = idx_to_icfl_factor[ls1_idx];
    let ls1_end_excl = if ls1_icfl_factor_idx < icfl_indexes.len() - 1 {
        icfl_indexes[ls1_icfl_factor_idx + 1]
    } else {
        str.len()
    };
    let ls1 = &str[ls1_idx..ls1_end_excl];

    let ls2_icfl_factor_idx = idx_to_icfl_factor[ls2_idx];
    let ls2_end_excl = if ls2_icfl_factor_idx < icfl_indexes.len() - 1 {
        icfl_indexes[ls2_icfl_factor_idx + 1]
    } else {
        str.len()
    };
    let ls2 = &str[ls2_idx..ls2_end_excl];

    // println!("Comparing: A={ls1:?}\tB={ls2:?}");

    // FIXME: works only with LSs of last ICFL factor
    /*if ls1_end_excl != str.len() || ls2_end_excl != str.len() {
        println!("WTF");
        exit(0x0100);
    }*/

    let mut i = 0;
    while i < ls1.len() && i < ls2.len() {
        if ls1[i] < ls2[i] {
            // ls1 < ls2
            return true;
        } else if ls1[i] > ls2[i] {
            // ls1 > ls2
            return false;
        }
        i += 1;
    }
    if i < ls1.len() {
        // "ls2" prefix of "ls1": ls1 > ls2.
        return false;
    }
    // FIXME: Assuming "ls_a_index" and "ls_b_index" are different
    // "ls1" prefix of "ls2": ...
    if ls1_icfl_factor_idx < icfl_indexes.len() - 1 {
        // ls1 > ls2
        return false;
    }
    true
}
