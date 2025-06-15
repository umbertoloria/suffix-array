use crate::factorization::get_max_factor_size;

pub fn create_tree<'a>(
    str: &'a [char],
    factor_indexes: &Vec<usize>,
    icfl_indexes: &Vec<usize>,
    idx_to_is_custom: &Vec<bool>,
) -> Tree<'a> {
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
            tree.add(ls_index, ls_size, false, str);
        }
        // LSs from Canonical Factors (from first to second-last ICFL Factors)
        for i in 0..icfl_indexes.len() - 1 {
            let next_icfl_factor_idx = icfl_indexes[i + 1];
            let curr_icfl_factor_size = next_icfl_factor_idx - icfl_indexes[i];
            if ls_size <= curr_icfl_factor_size {
                let ls_index = next_icfl_factor_idx - ls_size;
                tree.add(ls_index, ls_size, false, str);
            }
        }
        // LSs from Custom Factors
        for i in 0..factor_indexes.len() - 1 {
            let curr_factor_size = factor_indexes[i + 1] - factor_indexes[i];
            if ls_size <= curr_factor_size {
                let ls_index = factor_indexes[i + 1] - ls_size;
                if idx_to_is_custom[ls_index] {
                    tree.add(ls_index, ls_size, true, str);
                }
                // Else: Canonical Factor, already considered.
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
    pub fn add(&mut self, ls_index: usize, ls_size: usize, is_custom_ls: bool, str: &'a [char]) {
        self.root.add(ls_index, ls_size, 0, is_custom_ls, str);
    }
}

pub struct TreeNode<'a> {
    pub suffix_len: usize,
    pub rankings: Vec<usize>,
    pub children: Vec<(&'a [char], TreeNode<'a>)>,
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
        str: &'a [char],
    ) {
        if i_char == ls_size {
            self.update_rankings(ls_index, is_custom_ls, str);
            return;
        }

        let rest_of_ls = &str[ls_index + i_char..ls_index + ls_size];

        // Binary Search
        let mut p = 0;
        let mut q = self.children.len();
        while p < q {
            let mid = (q + p) / 2;
            let (mid_str, mid_node) = &mut self.children[mid];
            let mid_str = *mid_str;

            // Comparing "Mid. Str." with "Rest of LS".
            let mut i = 0;
            while i < rest_of_ls.len() && i < mid_str.len() {
                if rest_of_ls[i] != mid_str[i] {
                    break;
                }
                i += 1;
            }
            if i < rest_of_ls.len() && i < mid_str.len() {
                // Strings are different.
                if rest_of_ls[i] < mid_str[i] {
                    q = mid;
                } else {
                    // Then it's "rest_of_ls[i] > mid_str[i]".
                    p = mid + 1;
                }
            } else {
                // The case of "rest_of_ls" being prefix of "mid_str" is ignored.
                // Is up to the caller never to cause this case.
                mid_node.add(ls_index, ls_size, i_char + i, is_custom_ls, str);
                return;
            }
        }
        if p >= q {
            let mut new_node = TreeNode::new(ls_size);
            new_node.update_rankings(ls_index, is_custom_ls, str);
            self.children.insert(p, (rest_of_ls, new_node));
        }
    }
    fn update_rankings(&mut self, ls_index: usize, is_custom_ls: bool, str: &[char]) {
        if is_custom_ls {
            let custom_gs = &str[ls_index..];
            let idx = self.rankings.partition_point(|&gs_index| {
                let gs = &str[gs_index..];
                gs <= custom_gs
            });
            self.rankings.insert(idx, ls_index);
        } else {
            self.rankings.push(ls_index);
        }
    }
}
