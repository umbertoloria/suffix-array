use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;

pub fn rules_safe(
    x: usize,
    y: usize,
    child_offset: usize,
    str: &str,
    icfl_list: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    icfl_factor_list: &Vec<usize>,
    compare_cache: &mut CompareCache,
    monitor: &mut Monitor,
    slow_check: bool,
) -> bool {
    if !slow_check {
        rules(
            x,
            y,
            child_offset,
            str,
            icfl_list,
            is_custom_vec,
            icfl_factor_list,
            compare_cache,
            monitor,
        )
    } else {
        let cmp1_father = &str[x + child_offset..];
        let cmp2_child = &str[y + child_offset..];
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
            str,
            icfl_list,
            is_custom_vec,
            icfl_factor_list,
            compare_cache,
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
    icfl_list: &Vec<usize>,
    is_custom_vec: &Vec<bool>,
    icfl_factor_list: &Vec<usize>,
    compare_cache: &mut CompareCache,
    monitor: &mut Monitor,
) -> bool {
    let icfl_list_size = icfl_list.len();
    if is_custom_vec[x] && is_custom_vec[y] {
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
    } else if is_custom_vec[x] {
        monitor.new_compare_one_ls_in_custom_factor();
        if icfl_factor_list[x] <= icfl_factor_list[y] {
            monitor.new_compare_using_rules();
            if y >= icfl_list[icfl_list_size - 1] {
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
    } else if is_custom_vec[y] {
        monitor.new_compare_one_ls_in_custom_factor();
        if icfl_factor_list[y] <= icfl_factor_list[x] {
            monitor.new_compare_using_rules();
            if x >= icfl_list[icfl_list_size - 1] {
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
    } else if x >= icfl_list[icfl_list_size - 1] && y >= icfl_list[icfl_list_size - 1] {
        monitor.new_compare_using_rules();
        false
    } else if icfl_factor_list[x] == icfl_factor_list[y] {
        monitor.new_compare_using_rules();
        true
    } else {
        if x >= icfl_list[icfl_list_size - 1] {
            monitor.new_compare_using_rules();
            false
        } else if y >= icfl_list[icfl_list_size - 1] {
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
