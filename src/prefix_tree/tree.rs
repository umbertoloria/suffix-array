use crate::factorization::get_max_factor_size;
use crate::prefix_tree::monitor::Monitor;
use crate::prefix_tree::print::get_string_clone;
use crate::prefix_tree::rules::compatibility_property_icfl_a_before_b;
use std::process::exit;

pub fn create_tree(
    str: &[char],
    factor_indexes: &Vec<usize>,
    icfl_indexes: &Vec<usize>,
    idx_to_icfl_factor: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
    monitor: &mut Monitor,
) -> Tree {
    let str_length = str.len();
    let max_factor_size = get_max_factor_size(&factor_indexes, str_length);
    let last_icfl_factor_size = str_length - icfl_indexes[icfl_indexes.len() - 1];

    let mut tree = Tree::new();

    for ls_size in 1..=max_factor_size {
        // Looking for LSs with length "ls_size":
        // * first, LSs from Canonical Factors (sorted);
        // * then, LSs from Custom Factors.

        // LSs from Canonical Factors (last ICFL Factor)
        if ls_size <= last_icfl_factor_size {
            let ls_index = str_length - ls_size;
            tree.add(
                ls_index,
                ls_size,
                false,
                str,
                icfl_indexes,
                idx_to_icfl_factor,
                monitor,
            );

            // + Extra
            if cfg!(feature = "verbose") {
                tree.print(str);
            }
            // - Extra
        }
        // LSs from Canonical Factors (from first to second-last ICFL Factors)
        for i in 0..icfl_indexes.len() - 1 {
            let next_icfl_factor_idx = icfl_indexes[i + 1];
            let curr_icfl_factor_size = next_icfl_factor_idx - icfl_indexes[i];
            if ls_size <= curr_icfl_factor_size {
                let ls_index = next_icfl_factor_idx - ls_size;
                tree.add(
                    ls_index,
                    ls_size,
                    false,
                    str,
                    icfl_indexes,
                    idx_to_icfl_factor,
                    monitor,
                );

                // + Extra
                if cfg!(feature = "verbose") {
                    tree.print(str);
                }
                // - Extra
            }
        }
        // LSs from Custom Factors
        for i in 0..factor_indexes.len() - 1 {
            let curr_factor_size = factor_indexes[i + 1] - factor_indexes[i];
            if ls_size <= curr_factor_size {
                let ls_index = factor_indexes[i + 1] - ls_size;
                if idx_to_is_custom[ls_index] {
                    tree.add(
                        ls_index,
                        ls_size,
                        true,
                        str,
                        icfl_indexes,
                        idx_to_icfl_factor,
                        monitor,
                    );

                    // + Extra
                    if cfg!(feature = "verbose") {
                        tree.print(str);
                    }
                    // - Extra
                }
                // Else: Canonical Factor, already considered.
            }
        }
    }

    tree
}

pub struct Tree {
    pub root: TreeNode,
}
impl Tree {
    pub fn new() -> Self {
        Self {
            root: TreeNode::new(0),
        }
    }
    pub fn add(
        &mut self,
        ls_index: usize,
        ls_size: usize,
        is_custom_ls: bool,
        str: &[char],
        icfl_indexes: &Vec<usize>,
        idx_to_icfl_factor: &Vec<usize>,
        monitor: &mut Monitor,
    ) {
        self.root.add(
            ls_index,
            ls_size,
            0,
            is_custom_ls,
            str,
            icfl_indexes,
            idx_to_icfl_factor,
            monitor,
        );
    }
}

pub struct TreeNode {
    pub suffix_len: usize,
    pub rankings: Vec<usize>,
    pub children: Vec<((usize, usize), TreeNode)>,
}
impl TreeNode {
    pub fn new(suffix_len: usize) -> Self {
        Self {
            suffix_len,
            rankings: Vec::new(),
            children: Vec::new(),
        }
    }
    fn add(
        &mut self,
        ls_index: usize,
        ls_size: usize,
        i_char: usize,
        is_custom_ls: bool,
        str: &[char],
        icfl_indexes: &Vec<usize>,
        idx_to_icfl_factor: &Vec<usize>,
        monitor: &mut Monitor,
    ) {
        if i_char == ls_size {
            // + Extra
            if cfg!(feature = "verbose") {
                println!("   -> Populating node id=? with new ranking {ls_index}");
            }
            // - Extra

            self.update_rankings(ls_index, is_custom_ls, str, monitor);
            return;
        }

        let rest_of_ls_p = ls_index + i_char;
        let rest_of_ls_q = ls_index + ls_size;
        let rest_of_ls = &str[rest_of_ls_p..rest_of_ls_q];

        // + Extra
        if cfg!(feature = "verbose") {
            println!(
                " -> i_char={i_char} on REST={}, i_node=_, ls_index={ls_index}",
                get_string_clone(rest_of_ls),
            );
        }
        // - Extra

        // Binary Search
        let mut p = 0;
        let mut q = self.children.len();
        while p < q {
            let mid = (q + p) / 2;

            // + Extra
            if cfg!(feature = "verbose") {
                println!("   -> Binary Search: considering Mid Index={mid}");
            }
            // - Extra

            let (mid_label_pq, mid_node) = &mut self.children[mid];
            let (mid_label_p, mid_label_q) = *mid_label_pq;
            let mid_label = &str[mid_label_p..mid_label_q];

            // Comparing "Mid. Label" with "Rest of LS".
            // The case of "Mid. Label" being a prefix of "Rest of LS" is excluded as precondition.

            let mut preview_rest_of_ls_before_mid_str = None;
            if mid_label_q == str.len() {
                // Assuming "rest_of_ls_p < mid_label_p".
                /*if rest_of_ls_p >= mid_label_p {
                    println!("Should never happen!");
                    exit(0x0100);
                }*/

                // println!("Inserting ls_index={ls_index}, i_char={i_char}");

                preview_rest_of_ls_before_mid_str = Some(
                    //
                    compatibility_property_icfl_a_before_b(
                        rest_of_ls_p,
                        mid_label_p,
                        str,
                        icfl_indexes,
                        idx_to_icfl_factor,
                    ),
                );
            } else {
                // FIXME: approfondisci casi qui dentro...
                println!("SKIP Inserting ls_index={ls_index} to ls_size={ls_size} (i_char={i_char}): mid_label_p={mid_label_p}/mid_label_q={mid_label_q}");
            }
            /*if rest_of_ls_p < mid_label_p {
                // FIXME: è sempre così
                // println!("WOW!");
                preview_rest_of_ls_before_mid_str = Some(
                    //
                    compatibility_property_icfl_a_before_b(
                        rest_of_ls_p,
                        mid_label_p,
                        str,
                        icfl_indexes,
                        idx_to_icfl_factor,
                    ),
                );
            }*/

            if let Some(preview_rest_of_ls_before_mid_str) = preview_rest_of_ls_before_mid_str {
                // println!("COMPARING {rest_of_ls:?} with {mid_str:?}: wins {preview_rest_of_ls_before_mid_str}");

                // Strings are different.
                if preview_rest_of_ls_before_mid_str {
                    q = mid;
                    continue;
                } else {
                    // Then it's "rest_of_ls[i] > mid_str[i]".
                    // FIXME: può esserci un prefix, che serve per andare giù nell'albero
                    /*p = mid + 1;
                    continue;*/
                }
            }

            // TODO: Monitor string compare
            let mut i = 0;
            while i < rest_of_ls.len() && i < mid_label.len() {
                if rest_of_ls[i] != mid_label[i] {
                    break;
                }
                i += 1;
            }
            if i < rest_of_ls.len() && i < mid_label.len() {
                // Strings are different and "Rest of LS" is not prefix of "Mid. Label".

                // + Extra
                if cfg!(feature = "verbose") {
                    println!("     -> try another element");
                }
                // - Extra

                /*if let Some(preview_rest_of_ls_before_mid_str) = preview_rest_of_ls_before_mid_str {
                    // Strings are different.
                    let curr_test = rest_of_ls[i] < mid_label[i];
                    if preview_rest_of_ls_before_mid_str != curr_test {
                        println!("OMG: preview={preview_rest_of_ls_before_mid_str} but curr_test={curr_test}");
                        exit(0x0100);
                        // } else {
                        //     println!("OKOKOK");
                    }
                }*/
                if rest_of_ls[i] < mid_label[i] {
                    q = mid;
                } else {
                    // Then it's "rest_of_ls[i] > mid_label[i]".
                    p = mid + 1;
                }
            } else if i == rest_of_ls.len() && i == mid_label.len() {
                // Here "rest_of_ls == mid_label": node found!

                // + Extra
                if cfg!(feature = "verbose") {
                    println!("   -> Populating node id=? with new ranking {ls_index}");
                }
                // - Extra

                mid_node.update_rankings(ls_index, is_custom_ls, str, monitor);
                return;
            } else {
                // Here "mid_label prefix of rest_of_ls": continue on the next node...
                mid_node.add(
                    ls_index,
                    ls_size,
                    i_char + i,
                    is_custom_ls,
                    str,
                    icfl_indexes,
                    idx_to_icfl_factor,
                    monitor,
                );

                // The case of "rest_of_ls" being prefix of "mid_label" is ignored, so it can never
                // happen that "i < mid_label.len()": the caller should never cause this.
                return;
            }
        }
        if p >= q {
            let mut new_node = TreeNode::new(ls_size);
            new_node.update_rankings(ls_index, is_custom_ls, str, monitor);
            self.children
                .insert(p, ((rest_of_ls_p, rest_of_ls_q), new_node));

            // + Extra
            if cfg!(feature = "verbose") {
                let rest_of_ls_str = get_string_clone(rest_of_ls);
                if self.children.len() == 1 {
                    println!(
                        "   -> was empty, new node id=_ with prefix={} and ranking {}",
                        rest_of_ls_str, ls_index,
                    );
                } else {
                    println!(
                        "   -> found index p={p}, new node id=_ with prefix={} and ranking {}",
                        rest_of_ls_str, ls_index,
                    );
                }
            }
            // - Extra
        }
    }
    fn update_rankings(
        &mut self,
        ls_index: usize,
        is_custom_ls: bool,
        str: &[char],
        monitor: &mut Monitor,
    ) {
        if is_custom_ls {
            let custom_gs = &str[ls_index..];
            let idx = self.rankings.partition_point(|&gs_index| {
                let gs = &str[gs_index..];

                // + Extra
                // TODO: Monitor string compare
                monitor
                    .execution_outcome
                    .monitor_new_global_suffix_compare();
                // - Extra

                gs <= custom_gs
            });
            self.rankings.insert(idx, ls_index);
        } else {
            self.rankings.push(ls_index);
        }
    }
}
