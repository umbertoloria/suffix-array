use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_trie::rules::rules_safe;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;
use std::fs::{create_dir_all, File};
use std::io::Write;

pub struct PrefixTree {
    pub children: Vec<PrefixTreeNode>,
}
impl PrefixTree {
    pub fn print(&self, str: &str, prog_sa: &ProgSuffixArray) {
        println!("PrefixTree:");
        for child in &self.children {
            child.print(str, prog_sa, 1);
        }
    }

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
    pub fn print(&self, str: &str, prog_sa: &ProgSuffixArray, tabs_offset: usize) {
        let rankings = prog_sa.get_rankings(self.index);
        println!(
            "{}\"{}\" {:?}   m={} M={}",
            "\t".repeat(tabs_offset),
            self.get_label_from_first_ranking(str, rankings),
            rankings,
            if let Some(x) = self.min_father {
                format!("{}", x)
            } else {
                "-1".into()
            },
            if let Some(x) = self.max_father {
                format!("{}", x)
            } else {
                "-1".into()
            },
        );
        for child in &self.children {
            child.print(str, prog_sa, tabs_offset + 1);
        }
    }
    fn in_prefix_merge(
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
            child.in_prefix_merge_deep(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                self.index,
                compare_cache,
                monitor,
                verbose,
            );
        }
    }
    fn in_prefix_merge_deep(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        parent_index: usize,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        // Compare This Node's Rankings with Parent Node's Rankings.
        let parent_rankings = prog_sa.get_rankings(parent_index);

        let this_rankings = prog_sa.get_rankings(self.index);
        let this_first_ls_index = this_rankings[0];
        let this_ls_length = depths[this_first_ls_index];
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
            let parent_first_ls_index = parent_rankings[0];
            let parent_ls_length = depths[parent_first_ls_index];
            let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];
            println!(
                "Compare parent ({}) {:?} with curr ({}) {:?}",
                parent_ls, parent_rankings, this_ls, this_rankings
            );
        }

        // MERGE RANKINGS
        let mut i_parent = 0;

        while i_parent < parent_rankings.len() {
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            // TODO: Monitor string compare
            if curr_parent_ls < this_ls {
                // Go ahead, this part of Parent Rankings has LSs that are < than Curr LS.
                i_parent += 1;
            } else {
                // Found a Parent LS that is >= Curr LS.
                self.min_father = Some(i_parent);
                break;
            }
        }
        if i_parent >= parent_rankings.len() {
            // This means "min_father"=None and "max_father"=None.
        } else {
            // From here, we have a "min_father" value.

            // let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
            let curr_parent_ls_index = parent_rankings[i_parent];
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            // TODO: Monitor string compare
            if curr_parent_ls > this_ls {
                // This means "max_father"=None.
                // There's no Window for Comparing Rankings using "RULES".
            } else {
                while i_parent < parent_rankings.len() {
                    let curr_parent_ls_index = parent_rankings[i_parent];
                    let curr_parent_ls = &str[curr_parent_ls_index
                        ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
                    // TODO: Monitor string compare
                    if curr_parent_ls == this_ls {
                        // Go ahead, this part of Parent Rankings has LSs that are = than Curr LS.
                        self.max_father = Some(i_parent + 1);
                        i_parent += 1;
                    } else {
                        // Found a Parent LS that is > Curr LS.
                        break;
                    }
                }

                i_parent = self.min_father.unwrap();
                let mut j_this = 0;

                let mut new_rankings = Vec::new();
                if let Some(max_father) = self.max_father {
                    if verbose {
                        println!("   > start comparing, window=[{},{})", i_parent, max_father);
                    }
                    while i_parent < max_father && j_this < this_rankings.len() {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        let curr_this_ls_index = this_rankings[j_this];
                        let child_offset = self.suffix_len; // Could be inline.
                        let result_rules = rules_safe(
                            curr_parent_ls_index,
                            curr_this_ls_index,
                            child_offset,
                            str,
                            icfl_indexes,
                            &is_custom_vec,
                            &icfl_factor_list,
                            compare_cache,
                            monitor,
                            false,
                        );
                        if !result_rules {
                            if verbose {
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: father wins",
                                    &str
                                        [curr_parent_ls_index..curr_parent_ls_index + child_offset], curr_parent_ls_index, &str
                                        [curr_this_ls_index..curr_this_ls_index + child_offset], curr_this_ls_index, child_offset
                                );
                            }
                            new_rankings.push(curr_parent_ls_index);
                            i_parent += 1;
                        } else {
                            if verbose {
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
                                    &str
                                        [curr_parent_ls_index..curr_parent_ls_index + child_offset], curr_parent_ls_index, &str
                                        [curr_this_ls_index..curr_this_ls_index + child_offset], curr_this_ls_index, child_offset
                                );
                            }
                            new_rankings.push(curr_this_ls_index);
                            j_this += 1;
                        }
                    }
                }
                if j_this < this_rankings.len() {
                    // Enters in following while.
                } else {
                    if verbose {
                        println!("     > no child rankings left to add");
                    }
                }
                while j_this < this_rankings.len() {
                    let curr_this_ls_index = this_rankings[j_this];
                    let child_offset = self.suffix_len; // Could be inline.
                    if verbose {
                        println!(
                            "     > adding child=\"{}\" [{}], child.suff.len={}",
                            &str[curr_this_ls_index..curr_this_ls_index + child_offset],
                            curr_this_ls_index,
                            child_offset
                        );
                    }
                    new_rankings.push(curr_this_ls_index);
                    j_this += 1;
                }
                if let Some(max_father) = self.max_father {
                    while i_parent < max_father {
                        let curr_parent_ls_index = parent_rankings[i_parent];
                        let child_offset = self.suffix_len; // Could be inline.
                        if verbose {
                            println!(
                                "     > adding father=\"{}\" [{}], father.suff.len={}",
                                &str[curr_parent_ls_index..curr_parent_ls_index + child_offset],
                                curr_parent_ls_index,
                                child_offset
                            );
                        }
                        new_rankings.push(curr_parent_ls_index);
                        i_parent += 1;
                    }
                } else {
                    if verbose {
                        println!("     > no parent rankings left to add");
                    }
                }
                prog_sa.save_rankings_forced(self.index, new_rankings);
            }
        }

        // Now it's your turn to be the parent.
        for child in &mut self.children {
            child.in_prefix_merge_deep(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                self.index,
                compare_cache,
                monitor,
                verbose,
            );
        }
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

// SUFFIX ARRAY LOGGER
pub fn make_sure_directory_exist(folder_path: String) {
    create_dir_all(folder_path).unwrap();
}
pub fn log_suffix_array(sa: &Vec<usize>, filepath: String) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for sa_item in sa {
        file.write(format!("{}\n", sa_item).as_bytes())
            .expect("Unable to write");
    }
    file.flush().expect("Unable to flush file");
}
