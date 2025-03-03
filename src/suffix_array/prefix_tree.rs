use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_trie::{PrefixTrie, PrefixTrieData};
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
    pub rankings_forced: Option<Vec<usize>>,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTreeNode {
    fn get_rankings(&self, prog_sa: &ProgSuffixArray) -> Vec<usize> {
        // FIXME: Avoid cloning stuff
        if let Some(rankings_forced) = &self.rankings_forced {
            rankings_forced.to_vec()
        } else {
            prog_sa.get_rankings(self.index).to_vec()
        }
    }

    fn get_label_from_first_ranking<'a>(&self, str: &'a str, rankings: &Vec<usize>) -> &'a str {
        let first_ranking = rankings[0];
        &str[first_ranking..first_ranking + self.suffix_len]
    }
    pub fn print(&self, str: &str, prog_sa: &ProgSuffixArray, tabs_offset: usize) {
        let rankings = self.get_rankings(prog_sa);
        println!(
            "{}\"{}\" {:?}   m={} M={}",
            "\t".repeat(tabs_offset),
            self.get_label_from_first_ranking(str, &rankings),
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
        let this_ranking = self.get_rankings(prog_sa);
        for child in &mut self.children {
            child.in_prefix_merge_deep(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                &this_ranking,
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
        parent_rankings: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        // Compare this node's rankings with parent's rankings.
        let parent_first_ls_index = parent_rankings[0];
        let parent_ls_length = depths[parent_first_ls_index];
        let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];

        let this_rankings = &self.get_rankings(prog_sa);
        let this_first_ls_index = this_rankings[0];
        let this_ls_length = depths[this_first_ls_index];
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
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
                        let result_rules = Self::rules_safe(
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
                // FIXME: Update in Progressive Suffix Array
                self.rankings_forced = Some(new_rankings);
            }
        }

        // Now it's your turn to be the parent.
        let this_rankings = self.get_rankings(prog_sa);
        for child in &mut self.children {
            child.in_prefix_merge_deep(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                &this_rankings,
                compare_cache,
                monitor,
                verbose,
            );
        }
    }
    fn rules_safe(
        x: usize,
        y: usize,
        child_offset: usize,
        src: &str,
        icfl_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        icfl_factor_list: &Vec<usize>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        slow_check: bool,
    ) -> bool {
        if !slow_check {
            Self::rules(
                x,
                y,
                child_offset,
                src,
                icfl_list,
                is_custom_vec,
                icfl_factor_list,
                compare_cache,
                monitor,
            )
        } else {
            let cmp1_father = &src[x + child_offset..];
            let cmp2_child = &src[y + child_offset..];
            let mut oracle;
            if cmp1_father < cmp2_child {
                oracle = false; // Father first.
            } else {
                oracle = true; // Child first.
            }
            let given = Self::rules(
                x,
                y,
                child_offset,
                src,
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
        src: &str,
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
                src,
                y + child_offset,
                x + child_offset,
            )
            /*let cmp1 = &src[y + child_offset..];
            let cmp2 = &src[x + child_offset..];
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
                    src,
                    y + child_offset,
                    x + child_offset,
                )
                /*let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
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
                    src,
                    y + child_offset,
                    x + child_offset,
                )
                /*let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
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
                    src,
                    y + child_offset,
                    x + child_offset,
                )
                /*let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
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
                        src,
                        y + child_offset,
                        x + child_offset,
                    )
                    /*let cmp1 = &src[y + child_offset..];
                    let cmp2 = &src[x + child_offset..];
                    if cmp1 < cmp2 {
                        true
                    } else {
                        false
                    }*/
                }
            }
        }
    }
    fn get_common_prefix_partition(
        &mut self,
        str: &str,
        prog_sa: &ProgSuffixArray,
        verbose: bool,
    ) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();

        let common = self.get_rankings(prog_sa);

        if self.children.is_empty() {
            result.extend(common);
            if verbose {
                let rankings = self.get_rankings(prog_sa);
                println!(
                    "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                    self.get_label_from_first_ranking(str, &rankings),
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
            let rankings = self.get_rankings(prog_sa);
            println!(
                "Node {} (m={:?}, M={:?}) {:?} => {:?}",
                self.get_label_from_first_ranking(str, &rankings),
                self.min_father,
                self.max_father,
                rankings,
                result
            );
        }

        result
    }
}
pub fn create_prefix_tree_from_prefix_trie(
    root_trie: PrefixTrie,
    prog_sa: &mut ProgSuffixArray,
) -> PrefixTree {
    let (nodes_list, _) = create_prefix_tree_from_trie_deep(&root_trie, prog_sa, 0);
    let mut tree = PrefixTree {
        children: nodes_list,
    };
    // TODO: The Prefix Trie "root_trie" is going to be freed all in once here. Should we free as we
    //  build Prefix Tree instead?
    tree
}
fn create_prefix_tree_from_trie_deep(
    real_node: &PrefixTrie,
    prog_sa: &mut ProgSuffixArray,
    next_node_index: usize,
) -> (Vec<PrefixTreeNode>, usize) {
    let mut result = Vec::new();
    let mut next_node_index = next_node_index;

    if real_node.rankings_final.len() > 0 {
        // This Node has Rankings, so we consider it.

        // Create Prefix Tree Node
        prog_sa.assign_rankings_to_node_index(next_node_index, &real_node.rankings_final);
        let mut node = PrefixTreeNode {
            index: next_node_index,
            suffix_len: real_node.suffix_len,
            children: Vec::new(),
            rankings_forced: None,
            min_father: None,
            max_father: None,
        };
        next_node_index += 1;

        // Add children
        match &real_node.data {
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    let (nodes_list, next_node_index_) =
                        create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                    node.children.extend(nodes_list);
                    next_node_index = next_node_index_;
                }
            }
            PrefixTrieData::DirectChild((_, child_node)) => {
                let (nodes_list, next_node_index_) =
                    create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                node.children.extend(nodes_list);
                next_node_index = next_node_index_;
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }
        result.push(node);
    } else {
        // This Node is a Bridge, so we consider its Children (skipping Child Bridges).
        match &real_node.data {
            PrefixTrieData::Children(children) => {
                for (_, child_node) in children {
                    let (nodes_list, next_node_index_) =
                        create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                    result.extend(nodes_list);
                    next_node_index = next_node_index_;
                }
            }
            PrefixTrieData::DirectChild((_, child_node)) => {
                let (nodes_list, next_node_index_) =
                    create_prefix_tree_from_trie_deep(child_node, prog_sa, next_node_index);
                result.extend(nodes_list);
                next_node_index = next_node_index_;
            }
            PrefixTrieData::Leaf => {}
            PrefixTrieData::InitRoot => {}
        }
    }

    (result, next_node_index)
}

// PREFIX TREE LOGGER
pub fn log_prefix_tree(
    prefix_tree: &PrefixTree,
    str: &str,
    prog_sa: &ProgSuffixArray,
    filepath: String,
) {
    let mut file = File::create(filepath).expect("Unable to create file");
    for child in &prefix_tree.children {
        log_prefix_tree_recursive(child, str, prog_sa, &mut file, 0);
    }
    file.flush().expect("Unable to flush file");
}
fn log_prefix_tree_recursive(
    node: &PrefixTreeNode,
    str: &str,
    prog_sa: &ProgSuffixArray,
    file: &mut File,
    level: usize,
) {
    let rankings = &node.get_rankings(prog_sa);
    let mut line = format!(
        "{}{}",
        " ".repeat(level),
        node.get_label_from_first_ranking(str, rankings)
    );
    line.push_str(" [");
    let last_ranking = rankings[rankings.len() - 1];
    for i in 0..rankings.len() - 1 {
        let ranking = rankings[i];
        line.push_str(&format!("{}, ", ranking));
    }
    line.push_str(&format!("{}]", last_ranking));
    line.push_str("\n");
    file.write(line.as_bytes()).expect("Unable to write line");
    for child in &node.children {
        log_prefix_tree_recursive(child, str, prog_sa, file, level + 1);
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
