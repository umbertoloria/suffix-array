use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_tree::in_prefix_merge::IPMergeParams;

pub fn rules_safe(
    x: usize,
    y: usize,
    child_offset: usize,
    ip_merge_params: &mut IPMergeParams,
    monitor: &mut Monitor,
    slow_check: bool,
) -> bool {
    if !slow_check {
        rules(
            x,
            y,
            child_offset,
            ip_merge_params.str,
            ip_merge_params.icfl_indexes,
            ip_merge_params.idx_to_is_custom,
            ip_merge_params.idx_to_icfl_factor,
            ip_merge_params.compare_cache,
            monitor,
        )
    } else {
        let cmp1_father = &ip_merge_params.str[x + child_offset..];
        let cmp2_child = &ip_merge_params.str[y + child_offset..];
        let mut oracle;
        if cmp1_father < cmp2_child {
            oracle = false; // Father first.
        } else {
            oracle = true; // Child first.
        }
        let given = rules(
            x,
            y,
            child_offset,
            ip_merge_params.str,
            ip_merge_params.icfl_indexes,
            ip_merge_params.idx_to_is_custom,
            ip_merge_params.idx_to_icfl_factor,
            ip_merge_params.compare_cache,
            monitor,
        );
        if given != oracle {
            println!(
                " RULES: x={x:2}, y={y:2}, offset={child_offset} => {oracle}, BUT GIVEN WRONG!"
            );
        } else {
            // println!(" RULES: x={x:2}, y={y:2}, offset={child_offset} => {oracle}");
        }
        oracle
    }
}
fn rules(
    x: usize,
    y: usize,
    child_offset: usize,
    str: &str,
    icfl_indexes: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
    idx_to_icfl_factor: &Vec<usize>,
    compare_cache: &mut CompareCache,
    monitor: &mut Monitor,
) -> bool {
    let icfl_indexes_size = icfl_indexes.len();
    if idx_to_is_custom[x] && idx_to_is_custom[y] {
        monitor.new_compare_of_two_ls_in_custom_factors();
        monitor.new_compare_using_actual_string_compare();
        compare_cache.compare_1_before_2(
            //
            str,
            y + child_offset,
            x + child_offset,
        )
        /*let cmp1 = &str[y + child_offset..];
        let cmp2 = &str[x + child_offset..];
        if cmp1 < cmp2 {
            true
        } else {
            false
        }*/
    } else if idx_to_is_custom[x] {
        monitor.new_compare_one_ls_in_custom_factor();
        if idx_to_icfl_factor[x] <= idx_to_icfl_factor[y] {
            monitor.new_compare_using_rules();
            if y >= icfl_indexes[icfl_indexes_size - 1] {
                true
            } else {
                false
            }
        } else {
            monitor.new_compare_using_actual_string_compare();
            compare_cache.compare_1_before_2(
                //
                str,
                y + child_offset,
                x + child_offset,
            )
            /*let cmp1 = &str[y + child_offset..];
            let cmp2 = &str[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
        }
    } else if idx_to_is_custom[y] {
        monitor.new_compare_one_ls_in_custom_factor();
        if idx_to_icfl_factor[y] <= idx_to_icfl_factor[x] {
            monitor.new_compare_using_rules();
            if x >= icfl_indexes[icfl_indexes_size - 1] {
                false
            } else {
                true
            }
        } else {
            monitor.new_compare_using_actual_string_compare();
            compare_cache.compare_1_before_2(
                //
                str,
                y + child_offset,
                x + child_offset,
            )
            /*let cmp1 = &str[y + child_offset..];
            let cmp2 = &str[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
        }
    } else if x >= icfl_indexes[icfl_indexes_size - 1] && y >= icfl_indexes[icfl_indexes_size - 1] {
        monitor.new_compare_using_rules();
        false
    } else if idx_to_icfl_factor[x] == idx_to_icfl_factor[y] {
        monitor.new_compare_using_rules();
        true
    } else {
        if x >= icfl_indexes[icfl_indexes_size - 1] {
            monitor.new_compare_using_rules();
            false
        } else if y >= icfl_indexes[icfl_indexes_size - 1] {
            monitor.new_compare_using_actual_string_compare();
            compare_cache.compare_1_before_2(
                //
                str,
                y + child_offset,
                x + child_offset,
            )
            /*let cmp1 = &str[y + child_offset..];
            let cmp2 = &str[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }*/
        } else {
            if x > y {
                monitor.new_compare_using_rules();
                true
            } else {
                monitor.new_compare_using_actual_string_compare();
                compare_cache.compare_1_before_2(
                    //
                    str,
                    y + child_offset,
                    x + child_offset,
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
