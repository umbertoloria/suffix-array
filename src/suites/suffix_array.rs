use crate::factorization::icfl::icfl;
use crate::suffix_array::ls_and_rankings;
use crate::suffix_array::prefix_tree::create_prefix_tree_from_ls_and_rankings;

pub fn main_suffix_array() {
    let src = "aaabcaabcadcaabca";

    // Compute ICFL
    let factors = icfl(src);

    // Local Suffixes and Rankings
    let ls_and_rankings =
        ls_and_rankings::get_local_suffixes_and_rankings_from_icfl_factors(&factors);
    /*for s_index in 0..ls_and_rankings.count {
        let (s, s_ranking) = ls_and_rankings.get_s_and_ranking_by_index(s_index);
        println!("{s} -> {s_ranking:?}");
    }*/

    // Creating Prefix Tree
    let prefix_tree = create_prefix_tree_from_ls_and_rankings(&ls_and_rankings);
    prefix_tree.show_tree(0);
}
