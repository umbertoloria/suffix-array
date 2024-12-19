use crate::factorization::icfl::icfl;
use crate::suffix_array::ls_and_rankings;

pub fn main_suffix_array() {
    let src = "aaabcaabcadcaabca";
    let factors = icfl(src);

    let ls_and_rankings =
        ls_and_rankings::get_local_suffixes_and_rankings_from_icfl_factors(&factors);

    // Output
    for s_index in 0..ls_and_rankings.ls_list.len() {
        let s = &ls_and_rankings.ls_list[s_index];
        let s_ranking = &ls_and_rankings.ls_rankings[s_index];
        println!("{s} -> {s_ranking:?}");
    }
}
