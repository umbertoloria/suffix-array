use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::in_prefix_merge::IPMergeParams;

pub fn rules_safe(
    parent_ls_index: usize,
    child_ls_index: usize,
    child_ls_size: usize,
    ip_merge_params: &mut IPMergeParams,
    monitor: &mut Monitor,
    slow_check: bool,
) -> bool {
    if !slow_check {
        rules(
            parent_ls_index,
            child_ls_index,
            child_ls_size,
            ip_merge_params.str,
            ip_merge_params.icfl_indexes,
            ip_merge_params.idx_to_is_custom,
            ip_merge_params.idx_to_icfl_factor,
            ip_merge_params.compare_cache,
            monitor,
        )
    } else {
        let parent_ls = &ip_merge_params.str[parent_ls_index + child_ls_size..];
        let child_ls = &ip_merge_params.str[child_ls_index + child_ls_size..];
        let mut oracle = if parent_ls < child_ls {
            false // Father first.
        } else {
            true // Child first.
        };
        let given = rules(
            parent_ls_index,
            child_ls_index,
            child_ls_size,
            ip_merge_params.str,
            ip_merge_params.icfl_indexes,
            ip_merge_params.idx_to_is_custom,
            ip_merge_params.idx_to_icfl_factor,
            ip_merge_params.compare_cache,
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
    compare_cache: &mut CompareCache,
    monitor: &mut Monitor,
) -> bool {
    // Return values:
    //  FALSE => GS Parent < GS Child;
    //  TRUE  => GS Child < GS Parent.
    if idx_to_is_custom[parent_ls_index] && idx_to_is_custom[child_ls_index] {
        monitor.new_compare_of_two_ls_in_custom_factors();
        monitor.new_compare_using_actual_string_compare();
        return compare_cache.compare_1_before_2(
            //
            str,
            child_ls_index + child_ls_size,
            parent_ls_index + child_ls_size,
        );
        /*let cmp1 = &str[y + child_offset..];
        let cmp2 = &str[x + child_offset..];
        if cmp1 < cmp2 {
            true
        } else {
            false
        }*/
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
            compare_cache.compare_1_before_2(
                //
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
            /*let cmp1 = &str[y + child_offset..];
            let cmp2 = &str[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
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
            compare_cache.compare_1_before_2(
                //
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
            /*let cmp1 = &str[y + child_offset..];
            let cmp2 = &str[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
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
            compare_cache.compare_1_before_2(
                //
                str,
                child_ls_index + child_ls_size,
                parent_ls_index + child_ls_size,
            )
            /*let cmp1 = &str[y + child_offset..];
            let cmp2 = &str[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
        } else {
            if parent_ls_index > child_ls_index {
                monitor.new_compare_using_rules();
                true
            } else {
                monitor.new_compare_using_actual_string_compare();
                compare_cache.compare_1_before_2(
                    //
                    str,
                    child_ls_index + child_ls_size,
                    parent_ls_index + child_ls_size,
                )
                /*let cmp1 = &str[y + child_offset..];
                let cmp2 = &str[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }*/
            }
        }
    }
}
