use crate::suffix_array::compare_cache::CompareCache;
use crate::suffix_array::monitor::Monitor;
use crate::suffix_array::prefix_trie::PrefixTrie;
use crate::suffix_array::prog_suffix_array::ProgSuffixArray;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::exit;

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

    /*
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
    */
    pub fn shrink_up(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        icfl_factor_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        for child in &mut self.children {
            child.shrink_up(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                icfl_factor_list,
                is_custom_vec,
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
    fn get_rankings<'a>(&self, prog_sa: &'a ProgSuffixArray) -> &'a [usize] {
        prog_sa.get_rankings(self.index)
    }

    fn get_label_from_first_ranking<'a>(&self, str: &'a str, rankings: &[usize]) -> &'a str {
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
    /*
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
        // First layer. Here we have "A", "C", "G", "T".
        let mut start_from_this_p = prog_sa.get_rankings_p_q(self.index).0;
        for child in &mut self.children {
            // Second layer.
            start_from_this_p = child.in_prefix_merge_deep(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                self.index,
                start_from_this_p,
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
        start_from_parent_p: usize,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> usize {
        // TODO: Don't start from the beginning of Parent Rankings if you're not the First Child

        // Compare this node's rankings with parent's rankings.
        let (parent_p, parent_q) = prog_sa.get_rankings_p_q(parent_index);

        // MERGE RANKINGS
        // FIXME: sarebbe utile controllare "start_from_parent_p" rispetto a "parent_p"?
        let mut i_parent = start_from_parent_p;

        // let this_rankings = &self.get_rankings(prog_sa); // FIXME
        let (this_p, this_q) = prog_sa.get_rankings_p_q(self.index);
        let this_first_ls_index = prog_sa.get_ls_index(this_p);
        let this_ls_length = depths[this_first_ls_index];
        let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
        if verbose {
            // FIXME: sistema output
            let parent_first_ls_index = prog_sa.get_ls_index(parent_p);
            let parent_ls_length = depths[parent_first_ls_index];
            let parent_ls = &str[parent_first_ls_index..parent_first_ls_index + parent_ls_length];
            // let parent_rankings = prog_sa.get_rankings(parent_index); // FIXME
            let parent_rankings = prog_sa.get_rankings_manual(i_parent, parent_q);
            let this_rankings = prog_sa.get_rankings(self.index);
            println!(
                "Compare parent ({}) {:?} with curr ({}) {:?}",
                parent_ls, parent_rankings, this_ls, this_rankings
            );
        }

        while i_parent < parent_q {
            let curr_parent_ls_index = prog_sa.get_ls_index(i_parent);
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            // TODO: Monitor string compare
            if curr_parent_ls < this_ls {
                // Go ahead, this part of Parent Rankings has LSs that are < than Curr LS.
                i_parent += 1;
            } else {
                // Found a Parent LS that is >= Curr LS.
                // self.min_father = Some(i_parent); // FIXME
                self.min_father = Some(i_parent - parent_p);
                break;
            }
        }
        if i_parent >= parent_q {
            // This means "min_father"=None and "max_father"=None.
        } else {
            // From here, we have a "min_father" value.

            // let this_ls = &str[this_first_ls_index..this_first_ls_index + this_ls_length];
            let curr_parent_ls_index = prog_sa.get_ls_index(i_parent);
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
            // TODO: Monitor string compare
            if curr_parent_ls > this_ls {
                // This means "max_father"=None.
                // There's no Window for Comparing Rankings using "RULES".
                // prog_sa.update_rankings_child();
                // FIXME: deve essere operazione un po' diversa da "update_rankings_child"
                // FIXME: keep going from here...
            } else {
                while i_parent < parent_q {
                    let curr_parent_ls_index = prog_sa.get_ls_index(i_parent);
                    let curr_parent_ls = &str[curr_parent_ls_index
                        ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];
                    // TODO: Monitor string compare
                    if curr_parent_ls == this_ls {
                        // Go ahead, this part of Parent Rankings has LSs that are = than Curr LS.
                        // self.max_father = Some(i_parent + 1); // FIXME
                        self.max_father = Some(i_parent - parent_p + 1);
                        i_parent += 1;
                    } else {
                        // Found a Parent LS that is > Curr LS.
                        break;
                    }
                }

                // i_parent = self.min_father.unwrap(); // FIXME
                i_parent = self.min_father.unwrap() + parent_p;
                let mut j_this = this_p;

                let mut new_rankings = Vec::new();
                if let Some(mut max_father) = self.max_father {
                    if verbose {
                        println!("   > start comparing, window=[{},{})", i_parent, max_father);
                    }
                    while i_parent < max_father && j_this < this_q {
                        let curr_parent_ls_index = prog_sa.get_ls_index(i_parent);
                        let curr_this_ls_index = prog_sa.get_ls_index(j_this);
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
                                prog_sa.print(); // FIXME
                            }
                            prog_sa.update_rankings_child(
                                self.index,
                                j_this,
                                parent_index,
                                i_parent,
                            );
                            max_father -= 1; // FIXME: ripercussioni?
                                             // new_rankings.push(curr_parent_ls_index); // FIXME
                                             // i_parent += 1;
                            prog_sa.print(); // FIXME
                        } else {
                            if verbose {
                                println!(
                                    "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
                                    &str
                                        [curr_parent_ls_index..curr_parent_ls_index + child_offset], curr_parent_ls_index, &str
                                        [curr_this_ls_index..curr_this_ls_index + child_offset], curr_this_ls_index, child_offset
                                );
                                prog_sa.print(); // FIXME
                            }
                            // new_rankings.push(curr_this_ls_index); // FIXME
                            j_this += 1;
                        }
                    }
                }
                if j_this < this_q {
                    // Enters in following while.
                } else {
                    if verbose {
                        println!("     > no child rankings left to add");
                    }
                }
                while j_this < this_q {
                    let curr_this_ls_index = prog_sa.get_ls_index(j_this);
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
                    let mut max_i_parent = parent_p + max_father;
                    while i_parent < max_i_parent {
                        let curr_parent_ls_index = prog_sa.get_ls_index(i_parent);
                        let child_offset = self.suffix_len; // Could be inline.
                        if verbose {
                            println!(
                                "     > adding father=\"{}\" [{}], father.suff.len={}",
                                &str[curr_parent_ls_index..curr_parent_ls_index + child_offset],
                                curr_parent_ls_index,
                                child_offset
                            );
                        }
                        prog_sa.update_rankings_child(self.index, j_this, parent_index, i_parent);
                        // new_rankings.push(curr_parent_ls_index);
                        // i_parent += 1;
                        max_i_parent -= 1;
                        if verbose {
                            prog_sa.print();
                        }
                    }
                } else {
                    if verbose {
                        println!("     > no parent rankings left to add");
                    }
                }
            }
        }

        // Now it's your turn to be the parent.
        let mut start_from_this_p = prog_sa.get_rankings_p_q(self.index).0;
        for child in &mut self.children {
            start_from_this_p = child.in_prefix_merge_deep(
                str,
                prog_sa,
                depths,
                icfl_indexes,
                is_custom_vec,
                icfl_factor_list,
                self.index,
                start_from_this_p,
                compare_cache,
                monitor,
                verbose,
            );
        }

        i_parent
    }
    */
    fn shrink_up(
        &mut self,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        icfl_factor_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) {
        // First layer. Here we have "A", "C", "G", "T".

        if verbose {
            // FIXME: impr debug
            self.print(str, prog_sa, 0);
        }

        let (this_p, _) = prog_sa.get_rankings_p_q(self.index);
        let mut this_i = this_p;
        for child in &mut self.children {
            this_i = child.shrink_up_deep(
                self.index,
                this_i,
                str,
                prog_sa,
                depths,
                icfl_indexes,
                icfl_factor_list,
                is_custom_vec,
                compare_cache,
                monitor,
                verbose,
            );
        }
        self.children.clear();
    }
    fn shrink_up_deep(
        &mut self,
        parent_index: usize,
        parent_i: usize,
        str: &str,
        prog_sa: &mut ProgSuffixArray,
        depths: &mut Vec<usize>,
        icfl_indexes: &Vec<usize>,
        icfl_factor_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        compare_cache: &mut CompareCache,
        monitor: &mut Monitor,
        verbose: bool,
    ) -> usize {
        if verbose {
            println!("### SHRINK UP DEEP");
            println!(" : parent_index={parent_index} from parent_i={parent_i}");
            println!(" : self.index={}", self.index);
        }

        // Manage children
        let (this_p, _) = prog_sa.get_rankings_p_q(self.index);
        let mut curr_this_i = this_p;
        for child in &mut self.children {
            curr_this_i = child.shrink_up_deep(
                self.index,
                curr_this_i,
                str,
                prog_sa,
                depths,
                icfl_indexes,
                icfl_factor_list,
                is_custom_vec,
                compare_cache,
                monitor,
                verbose,
            );
        }

        // Manage this with parent
        if verbose {
            // FIXME: impr debug
            println!(" CHECKING FROM SELF NODE INDEX={}", self.index);
        }
        let mut curr_parent_i = parent_i;
        let (_, mut parent_q) = prog_sa.get_rankings_p_q(parent_index);

        // FIXME: they may be changed up...
        let (this_p, this_q) = prog_sa.get_rankings_p_q(self.index);
        let mut curr_this_i = this_p;
        if this_p == this_q {
            // FIXME: node without rankings?
            exit(0x0100);
        }
        // let this_ls_length = self.suffix_len;
        let this_first_ls_index = prog_sa.get_ls_index(curr_this_i);
        // let this_ls_length = depths[this_first_ls_index]; // FIXME: let's try
        let this_ls_length = self.suffix_len;

        // Skip all Parent LSs that are < than First This LS.
        while curr_parent_i < parent_q && curr_this_i < this_q {
            if verbose {
                // FIXME: impr debug
                println!(
                    " Iteration: Parent I={}, This I={}, This LS Length={}",
                    curr_parent_i, curr_this_i, this_ls_length
                );
                prog_sa.print();
            }

            let curr_parent_ls_index = prog_sa.get_ls_index(curr_parent_i);
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + this_ls_length, str.len())];

            let curr_this_ls_index = prog_sa.get_ls_index(curr_this_i);
            let curr_this_ls = &str[curr_this_ls_index..curr_this_ls_index + this_ls_length];

            // TODO: Monitor string compare
            if curr_parent_ls < curr_this_ls {
                // Go ahead, this part of Parent Rankings has LSs that are < than Curr LS.
                curr_parent_i += 1;
                if verbose {
                    // FIXME: impr debug
                    println!(
                        " > Parent Curr LS \"{}\" ({}) is smaller than \"{}\" ({}): go check the next one",
                        curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index
                    );

                    // FIXME: questo non andava, in console:
                    //  "e father="GA" [16] <-> child="GA" [26], child.suff.len=2: child w"
                    //
                }
            } else {
                // Found a Parent LS that is >= Curr LS.
                if verbose {
                    // FIXME: impr debug
                    println!(
                        " > Parent Curr LS \"{}\" ({}) >= This Curr LS \"{}\" ({}): found!",
                        curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index
                    );
                }
                break;
            }
        }
        if curr_parent_i >= parent_q {
            if verbose {
                // FIXME: impr debug
                println!("   > parent LSs finished: {curr_parent_i} => {parent_q}");
                println!("   > all This Node LSs will be inserted at the end of Parent LSs list");
            }
            // FIXME: all Parent LSs < This Child LSs => this node's LSs will be inserted at the end
            //  of the Parent LSs list
            curr_parent_i =
                prog_sa.update_rankings_parent_including_all_child_lss(parent_index, self.index);
            if verbose {
                prog_sa.print();
                println!("   > from now (A), curr_parent_i={curr_parent_i}");
            }
            return curr_parent_i;
        }
        if curr_this_i >= this_q {
            // FIXME: finiti This Child LSs, quindi dichiara inglobato e vai ad altri fratelli a dx
            if verbose {
                // FIXME: impr debug
                println!("   > (***) finito child: {} => {}", curr_this_i, this_q);
            }
            return curr_parent_i;
        }
        // FIXME: da ora in poi, il Parent Curr LS >= This Curr LS
        // FIXME: ok, abbiamo una prima coppia su cui usare "RULES"
        while curr_parent_i < parent_q && curr_this_i < this_q {
            let curr_this_ls_index = prog_sa.get_ls_index(curr_this_i);
            // let deep_this_ls_length = depths[curr_this_ls_index]; // FIXME: doesn't work...
            let deep_this_ls_length = this_ls_length;
            let curr_this_ls = &str[curr_this_ls_index..curr_this_ls_index + deep_this_ls_length];

            if parent_i == 22 {
                println!("******************");
            }
            let curr_parent_ls_index = prog_sa.get_ls_index(curr_parent_i);
            let curr_parent_ls = &str[curr_parent_ls_index
                ..usize::min(curr_parent_ls_index + deep_this_ls_length, str.len())];

            // TODO: Monitor string compare
            if curr_parent_ls > curr_this_ls {
                if verbose {
                    println!(
                        " > Parent Curr LS \"{}\" ({}) is greater than \"{}\" ({}), including all Self LSs before it",
                        curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index
                    );
                }

                // FIXME: devi inglobare tutti i This LSs rimasti prima di questo Curr Parent LS, e
                //  vedere il prossimo nodo a destra

                if verbose {
                    // prog_sa.print();
                }
                let child_rankings_moved = prog_sa
                    .update_rankings_parent_including_all_child_lss_before_curr_parent_ls(
                        parent_index,
                        curr_parent_i,
                        self.index,
                        verbose,
                    );
                // FIXME: bisogna aggiornare "parent_q" anche qui, perché è stato appena modificato
                let (_, new_parent_q) = prog_sa.get_rankings_p_q(parent_index);
                parent_q = new_parent_q;

                // We return the new position of the Parent LS (the one that was at the left of all the
                // items that we just moved).
                curr_parent_i += child_rankings_moved;
                curr_this_i += child_rankings_moved;

                if verbose {
                    prog_sa.print();
                    println!("   > from now (B), curr_parent_i={curr_parent_i}");
                }

                // break; // FIXME: è saggio commentare questo?
            } else if curr_parent_ls == curr_this_ls {
                // FIXME: using rules to do comparison

                // FIXME: ricorda che il confronto può skippare i primi elementi visto che abbiamo
                //  appena fatto il confronto LS
                let result_rules = Self::rules_safe(
                    curr_parent_ls_index,
                    curr_this_ls_index,
                    deep_this_ls_length, // FIXME: usa depths
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
                            curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index, deep_this_ls_length
                        );
                        prog_sa.print(); // FIXME
                    }
                    // FIXME: tutto ok, Parent Curr GS is < This Curr GS, to on...
                    curr_parent_i += 1;
                    if verbose {
                        prog_sa.print(); // FIXME
                    }
                } else {
                    if verbose {
                        if curr_parent_ls_index == 16 && curr_this_ls_index == 26 {
                            println!("-----------------");
                            println!(" 1. {}", depths[curr_parent_ls_index]);
                            println!(" 2. {}", depths[curr_this_ls_index]);
                            println!(" 3. deep_this_ls_length={}", deep_this_ls_length);
                        }
                        println!(
                            "     > compare father=\"{}\" [{}] <-> child=\"{}\" [{}], child.suff.len={}: child wins",
                            curr_parent_ls, curr_parent_ls_index, curr_this_ls, curr_this_ls_index, deep_this_ls_length
                        );
                        prog_sa.print(); // FIXME
                    }
                    // FIXME: should swap Parent Curr I and This Curr I because:
                    //  Parent Curr GS > This Curr GS
                    prog_sa.update_rankings_child(
                        self.index,
                        curr_this_i,
                        parent_index,
                        curr_parent_i,
                    );
                    // FIXME: bisogna aggiornare "parent_q" anche qui, perché è stato appena modificato
                    let (_, new_parent_q) = prog_sa.get_rankings_p_q(parent_index);
                    parent_q = new_parent_q;
                    curr_parent_i += 1;
                    curr_this_i += 1;

                    if verbose {
                        prog_sa.print(); // FIXME
                    }
                }
            } else {
                // FIXME: shuold never happen.
                println!("********************");
                exit(0x0100);
            }
        }
        if curr_this_i < this_q {
            // FIXME: qui ci siamo perché tutti i Parent LSs sono stati gestiti ma la lista di
            //  This LSs non è ancora vuota. Ciò vuol dire che dobbiamo inglobare.

            println!("   > having curr_this_i={curr_this_i} < this_q={this_q}");
            println!("     (some remaining This Node LSs will be added at the end of Parent LSs)");
            if verbose {
                // prog_sa.print();
            }
            let child_rankings_moved = prog_sa
                .update_rankings_parent_including_all_child_lss_before_curr_parent_ls(
                    parent_index,
                    curr_parent_i,
                    self.index,
                    verbose,
                );
            // FIXME: bisogna aggiornare "parent_q" anche qui, perché è stato appena modificato
            let (_, new_parent_q) = prog_sa.get_rankings_p_q(parent_index);
            parent_q = new_parent_q;

            // We return the new position of the Parent LS (the one that was at the left of all the
            // items that we just moved).
            curr_parent_i += child_rankings_moved;
            curr_this_i += child_rankings_moved;

            // FIXME: tecnicamente qui è inutile fare questi ultimi tre aggiornamenti, però...

            if verbose {
                prog_sa.print();
                println!("   > from now (C), curr_parent_i={curr_parent_i}");
            }
        } else {
            // All LSs of This Node have been inherited in Parent Node.
        }

        self.children.clear();

        curr_parent_i
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
    let (nodes_list, _) = create_prefix_tree_from_trie_deep(root_trie, prog_sa, 0);
    let mut tree = PrefixTree {
        children: nodes_list,
    };
    tree
}
fn create_prefix_tree_from_trie_deep(
    real_node: PrefixTrie,
    prog_sa: &mut ProgSuffixArray,
    next_node_index: usize,
) -> (Vec<PrefixTreeNode>, usize) {
    let mut result = Vec::new();
    let mut next_node_index = next_node_index;

    if real_node.rankings_final.len() > 0 {
        // This Node has Rankings, so we consider it.

        // Create Prefix Tree Node
        prog_sa.assign_rankings_to_node_index(next_node_index, real_node.rankings_final);
        let mut node = PrefixTreeNode {
            index: next_node_index,
            suffix_len: real_node.suffix_len,
            children: Vec::new(),
            min_father: None,
            max_father: None,
        };
        next_node_index += 1;

        // Add children
        for (_, son) in real_node.sons {
            let (nodes_list, next_node_index_) =
                create_prefix_tree_from_trie_deep(son, prog_sa, next_node_index);
            node.children.extend(nodes_list);
            next_node_index = next_node_index_;
        }
        result.push(node);
    } else {
        // This Node is a Bridge, so we consider its Children (skipping Child Bridges).
        for (_, son) in real_node.sons {
            let (nodes_list, next_node_index_) =
                create_prefix_tree_from_trie_deep(son, prog_sa, next_node_index);
            result.extend(nodes_list);
            next_node_index = next_node_index_;
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
