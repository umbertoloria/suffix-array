use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;

pub struct PrefixTree {
    pub children: Vec<PrefixTreeNode>,
}
impl PrefixTree {
    pub fn in_prefix_merge(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        for child in &mut self.children {
            child.in_prefix_merge(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                compare_cache,
                monitor,
                verbose,
            );
        }
    }
    pub fn prepare_get_common_prefix_partition(
        &mut self,
        sa: &mut Vec<usize>,
        str: &str,
        prog_sa: &ProgSuffixArray,
        verbose: bool,
    ) {
        for first_layer_child in &mut self.children {
            sa.extend(first_layer_child.get_common_prefix_partition(str, prog_sa, verbose));
        }
    }
}
pub struct PrefixTreeNode {
    pub index: usize,
    pub suffix_len: usize,
    pub children: Vec<PrefixTreeNode>,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTreeNode {
    pub fn get_label_from_first_ranking<'a>(&self, str: &'a str, rankings: &[usize]) -> &'a str {
        // Unfortunately (maybe), each caller has its own reason to already have "rankings"...
        let first_ranking = rankings[0];
        &str[first_ranking..first_ranking + self.suffix_len]
    }
    fn get_common_prefix_partition(
        &mut self,
        str: &str,
        prog_sa: &ProgSuffixArray,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        let common = prog_sa.get_rankings(self.index);

        if self.children.is_empty() {
            result.extend(common);
            if verbose {
                // let rankings = self.get_rankings(prog_sa); // Before it was...
                let rankings = common;
                println!(
                    "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                    self.get_label_from_first_ranking(str, rankings),
                    self.min_father,
                    self.max_father,
                    rankings,
                    result
                );
            }
            return result;
        }

        let mut position = 0;
        for child in &mut self.children {
            let temp = child.get_common_prefix_partition(str, prog_sa, verbose);
            if let Some(min_father) = child.min_father {
                if verbose {
                    println!(
                        "Here self=?? and child=??",
                        // self.get_label(str),
                        // child.get_label(str)
                    );
                }
                if min_father >= position {
                    result.extend(&common[position..min_father]);
                }
                result.extend(temp);
                if let Some(max_father) = child.max_father {
                    position = max_father;
                } else {
                    position = min_father;
                }
            } else {
                // Min Father is None.
                result.extend(&common[position..]);
                result.extend(temp);
                position = common.len();
            }
        }
        result.extend(&common[position..]);

        if verbose {
            let rankings = prog_sa.get_rankings(self.index);
            println!(
                "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                self.get_label_from_first_ranking(str, rankings),
                self.min_father,
                self.max_father,
                rankings,
                result
            );
        }

        result
    }
}
