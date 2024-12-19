use crate::factorization::icfl::icfl;
use std::collections::HashMap;

pub fn main_suffix_array() {
    let src = "aaabcaabcadcaabca";
    let factors = icfl(src);

    // Factor offsets
    let mut factor_offsets = Vec::new();
    let mut offset = 0;
    for i in 0..factors.len() {
        factor_offsets.push(offset);
        offset += factors[i].len();
    }

    let mut ls_list = Vec::new();
    let mut ls_rankings_map: HashMap<String, Vec<usize>> = HashMap::new();
    // Closure to address one factor's Local Suffixes
    let mut compute_factor_local_suffixes = |i_factor: usize| {
        let factor = &factors[i_factor];
        for i in (0..factor.len()).rev() {
            // Local suffix "s"
            let s = &factor[i..factor.len()];

            // Add "s" to Local Suffixes list
            if !ls_list.contains(&s) {
                ls_list.push(s);
            }

            // Add [s]-ranking
            let index_in_whole_string = factor_offsets[i_factor] + i;
            if !ls_rankings_map.contains_key(s) {
                ls_rankings_map.insert(s.into(), Vec::new());
            }
            ls_rankings_map
                .get_mut(s)
                .unwrap()
                .splice(0..0, [index_in_whole_string]);
        }
    };

    // Computing all factors
    for i in (0..factors.len() - 1).rev() {
        compute_factor_local_suffixes(i);
    }
    compute_factor_local_suffixes(factors.len() - 1);

    // Finalizing "ls_list" and "ls_rankings"
    ls_list.sort();
    let mut ls_rankings_list = Vec::new();
    for s_index in 0..ls_list.len() {
        let s = ls_list[s_index];
        let s_ranking = ls_rankings_map.get(s).unwrap();
        ls_rankings_list.push(s_ranking);
    }

    // Output
    for s_index in 0..ls_list.len() {
        let s = ls_list[s_index];
        let s_ranking = ls_rankings_map.get(s).unwrap();
        println!("{s} -> {s_ranking:?}");
    }
}
