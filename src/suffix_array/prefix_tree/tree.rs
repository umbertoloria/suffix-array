use crate::suffix_array::chunking::get_max_factor_size;
use crate::suffix_array::monitor::Monitor;
use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub fn create_tree<'a>(
    s_bytes: &'a [u8],
    icfl_indexes: &Vec<usize>,
    chunk_size: Option<usize>,
    monitor: &mut Monitor,
) -> Tree<'a> {
    let str_length = s_bytes.len();
    let mut max_factor_size = get_max_factor_size(&icfl_indexes, str_length);
    if let Some(chunk_size) = chunk_size {
        if chunk_size < max_factor_size {
            max_factor_size = chunk_size;
        }
    }
    let chunk_size_real = chunk_size.unwrap_or(0);
    let last_icfl_factor_size = str_length - icfl_indexes[icfl_indexes.len() - 1];

    let mut tree = Tree::new();

    for ls_size in 1..=max_factor_size {
        // Every iteration looks for all Custom Factors whose length is <= "ls_size" and, if there
        // exist, takes their Local Suffixes of "ls_size" length.

        // Last ICFL Factor
        if ls_size <= last_icfl_factor_size {
            let ls_index = str_length - ls_size;
            tree.add(ls_index, ls_size, false, s_bytes, monitor);
            if cfg!(feature = "verbose") {
                tree.print();
            }
        }

        // All ICFL Factors from first to second-last
        for i_factor in 0..icfl_indexes.len() - 1 {
            let next_icfl_factor_index = icfl_indexes[i_factor + 1];
            let curr_icfl_factor_size = next_icfl_factor_index - icfl_indexes[i_factor];
            if ls_size <= curr_icfl_factor_size {
                let ls_index = next_icfl_factor_index - ls_size;
                tree.add(ls_index, ls_size, false, s_bytes, monitor);
                if cfg!(feature = "verbose") {
                    tree.print();
                }
            }
        }

        if ls_size <= chunk_size_real {
            // Custom Factorization is active.
            for i_factor in 0..icfl_indexes.len() {
                let curr_icfl_factor_index = icfl_indexes[i_factor];

                // Curr ICFL Factor Size
                let curr_icfl_factor_size = if i_factor < icfl_indexes.len() - 1 {
                    icfl_indexes[i_factor + 1]
                } else {
                    str_length
                } - curr_icfl_factor_index;

                if curr_icfl_factor_size < ls_size {
                    continue;
                }

                /// let remaining_space_for_chunking = curr_icfl_factor_size - chunk_size_real;
                // Updating "factor_indexes"
                // Es. on the 2nd factor "B": curr_icfl_factor_index=3, curr_icfl_factor_size=1
                if curr_icfl_factor_size > chunk_size_real {
                    let first_chunk_index_offset = curr_icfl_factor_size % chunk_size_real;
                    let mut curr_cust_fact_index =
                        curr_icfl_factor_index + first_chunk_index_offset;
                    if first_chunk_index_offset > 0 {
                        // Example: ICFL Factor is "DCAABCA", Chunk Size is 4, so Custom Factors
                        // are "DCA" and "ABCA". In this case, First Chunk Index Offset = 3, since
                        // the first Custom Factor has Length < Chunk Size.
                        if ls_size <= first_chunk_index_offset {
                            let ls_index = curr_cust_fact_index - ls_size;
                            tree.add(ls_index, ls_size, true, s_bytes, monitor);
                            if cfg!(feature = "verbose") {
                                tree.print();
                            }
                        }
                    }
                    // Avoid "last_cust_fact_index" because all its LSs are LSs of ICFL Factors.
                    let last_cust_fact_index =
                        curr_icfl_factor_index + curr_icfl_factor_size - chunk_size_real;
                    while curr_cust_fact_index < last_cust_fact_index {
                        let ls_index = curr_cust_fact_index + chunk_size_real - ls_size;
                        tree.add(ls_index, ls_size, true, s_bytes, monitor);
                        if cfg!(feature = "verbose") {
                            tree.print();
                        }
                        curr_cust_fact_index += chunk_size_real;
                    }
                }
            }
        }
    }

    tree
}

pub struct Tree<'a> {
    pub root: TreeNode<'a>,
}
impl<'a> Tree<'a> {
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
        s_bytes: &'a [u8],
        monitor: &mut Monitor,
    ) {
        self.root
            .add(ls_index, ls_size, 0, is_custom_ls, s_bytes, monitor);
    }

    // PRINT
    pub fn print(&self) {
        self.print_node(&self.root, 0, "");
    }
    fn print_node(&self, self_node: &TreeNode<'a>, tabs_offset: usize, self_label: &str) {
        println!(
            "{}|{:2}: \"{}\" {}",
            "\t".repeat(tabs_offset),
            tabs_offset,
            self_label,
            format!("{:?}", self_node.rankings),
        );
        for (child_node_prefix, child_node) in &self_node.children {
            let child_node_prefix = *child_node_prefix;
            let prefix_str = get_string_clone(child_node_prefix);
            let child_node_label = format!("{}{}", self_label, prefix_str);
            self.print_node(child_node, tabs_offset + 1, &child_node_label);
        }
    }
}

pub struct TreeNode<'a> {
    pub suffix_len: usize,
    pub rankings: Vec<usize>,
    pub children: Vec<(&'a PrefixTreeString, TreeNode<'a>)>,
}
impl<'a> TreeNode<'a> {
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
        s_bytes: &'a [u8],
        monitor: &mut Monitor,
    ) {
        if i_char == ls_size {
            if cfg!(feature = "verbose") {
                println!("   -> Populating node id=? with new ranking {ls_index}");
            }
            self.update_rankings(ls_index, is_custom_ls, s_bytes, monitor);
            return;
        }

        let rest_of_ls = &s_bytes[ls_index + i_char..ls_index + ls_size];

        if cfg!(feature = "verbose") {
            println!(
                " -> i_char={i_char} on REST={}, i_node=_, ls_index={ls_index}",
                get_string_clone(rest_of_ls),
            );
        }

        // Binary Search
        let mut p = 0;
        let mut q = self.children.len();
        while p < q {
            let mid = (q + p) / 2;
            if cfg!(feature = "verbose") {
                println!("   -> Binary Search: considering Mid Index={mid}");
            }
            let (mid_str, mid_node) = &mut self.children[mid];
            let mid_str = *mid_str;

            // Comparing "Mid. Str." with "Rest of LS".
            // TODO: Monitor string compare
            let mut i = 0;
            while i < rest_of_ls.len() && i < mid_str.len() {
                if rest_of_ls[i] != mid_str[i] {
                    break;
                }
                i += 1;
            }
            if i < rest_of_ls.len() && i < mid_str.len() {
                if cfg!(feature = "verbose") {
                    println!("     -> try another element");
                }
                // Strings are different.
                if rest_of_ls[i] < mid_str[i] {
                    q = mid;
                } else {
                    // Then it's "rest_of_ls[i] > mid_str[i]".
                    p = mid + 1;
                }
            } else {
                // Due to how this Tree structure is used, all the code commented below can be
                // simplified in this code here. Still, it's kept for further explanation.

                mid_node.add(
                    ls_index,
                    ls_size,
                    i_char + i,
                    is_custom_ls,
                    s_bytes,
                    monitor,
                );
                return;

                /*
                // TOD: Simplify branch nidification
                // Here it can be true either:
                // 1. Strings are the same, or
                // 2. Strings have some prefix relation.
                if i == rest_of_ls.len() && i == mid_str.len() {
                    // Case 1. Strings are the same.
                    if cfg!(feature = "verbose") {
                        println!("     -> Case 1: found final Node={mid_node_id}");
                    }
                    i_node = mid_node_id;
                    i_char += i;
                    break;
                } else {
                    if cfg!(feature = "verbose") {
                        println!("     -> Case 2");
                    }
                    // Case 2. It can be either:
                    // 2A. "mid_str" is prefix of "rest_of_ls", or
                    // 2B. "rest_of_ls" is prefix of "mid_str".
                    if i < rest_of_ls.len() {
                        if cfg!(feature = "verbose") {
                            println!(
                                "       -> Case 2A: mid_str={} prefix of rest_of_ls={}",
                                get_string_clone(mid_str),
                                get_string_clone(rest_of_ls),
                            );
                        }
                        // Case 2A. We have that "mid_str" is prefix of "rest_of_ls".
                        i_node = mid_node_id;
                        i_char += i;
                        break;
                    } else {
                        // Then it's "i < mid_str.len()".
                        // Case 2B. We have that "rest_of_ls" is prefix of "mid_str".
                        if cfg!(feature = "verbose") {
                            println!(
                                "       -> Case 2B: rest_of_ls={} prefix of mid_str={}",
                                get_string_clone(rest_of_ls),
                                get_string_clone(mid_str),
                            );
                        }
                        // TOD: Here, we should update this Edge (that has string "mid_str")
                        //  to use the string "rest_of_ls", since "rest_of_ls" is prefix of
                        //  "mid_str". Here we avoid coding this since the callee of this
                        //  function already knows that has to add first all LSs with length "i"
                        //  and then all LSs with length "i+1", so this case should never happen
                        //  (if this ordering is followed).
                        // Example: In the Tree there was "\0" [] -> "CA" [10, 4], and now we
                        // want to insert a node "C" for ranking 6.
                        // The result should be "\0" [] -> "C" [6] -> "CA" [10, 4].
                        // What we have here instead is: "\0" [] -> "CA" [10, 4]
                        //  and "\0" [] -> "C" [6], so "CA" and "C" are siblings. But actually
                        // "C" should be inserted as the *new father* of "CA" (even if "CA" was
                        // already there).
                        let new_node_id = self.create_node(i_char + rest_of_ls.len());
                        curr_node.children.insert(mid, (rest_of_ls, new_node_id));
                        i_node = new_node_id;
                        i_char += rest_of_ls.len();
                        break;
                    }
                }
                */
            }
        }
        if p >= q {
            let mut new_node = TreeNode::new(ls_size);
            new_node.update_rankings(ls_index, is_custom_ls, s_bytes, monitor);
            self.children.insert(p, (rest_of_ls, new_node));

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
        }
    }
    fn update_rankings(
        &mut self,
        ls_index: usize,
        is_custom_ls: bool,
        s_bytes: &[u8],
        monitor: &mut Monitor,
    ) {
        monitor
            .execution_outcome
            .monitor_update_rankings(ls_index, is_custom_ls);

        if is_custom_ls {
            let custom_gs = &s_bytes[ls_index..];
            let idx = self.rankings.partition_point(|&gs_index| {
                let gs = &s_bytes[gs_index..];

                // TODO: Monitor string compare
                monitor
                    .execution_outcome
                    .monitor_new_global_suffix_compare();

                gs <= custom_gs
            });
            self.rankings.insert(idx, ls_index);
            // Duplicated code: look for (*njk).
        } else {
            self.rankings.push(ls_index);
        }
    }
}

// STRING ABSTRACTION
type PrefixTreeString = [u8];
pub fn get_string_clone(str_type: &PrefixTreeString) -> String {
    // TODO: Needs cloning
    let cloned_vec = str_type.to_vec();
    String::from_utf8(cloned_vec).unwrap()
}
