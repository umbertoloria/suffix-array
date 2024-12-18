use crate::factorization::icfl::icfl;
use std::collections::HashMap;

pub fn main_suffix_array() {
    let src = "aaabcaabcadcaabca";
    let factors = icfl(src);
    println!("ICFL: {:?}", factors);

    let mut local_suffixes = Vec::new();
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();

    // FACTOR OFFSETS
    let mut factor_offsets = Vec::new();
    let mut offset = 0;
    for i in 0..factors.len() {
        factor_offsets.push(offset);
        offset += factors[i].len();
    }

    // Closure to address one factor's Local Suffixes
    let mut compute_factor_local_suffixes = |i_factor: usize| {
        let factor = &factors[i_factor];
        // println!("managing {}", factor);
        for i in (0..factor.len()).rev() {
            let local_suffix = &factor[i..factor.len()];
            // println!(" -> {} {} -> {}", i, factor.len(), local_suffix);
            if !local_suffixes.contains(&local_suffix) {
                local_suffixes.push(local_suffix);
            }
            if !map.contains_key(local_suffix) {
                map.insert(local_suffix.to_string(), Vec::new());
            }
            let index_in_whole_string = factor_offsets[i_factor] + i;
            map.get_mut(local_suffix)
                .unwrap()
                .splice(0..0, [index_in_whole_string]);
        }
    };

    // Computing all factors
    for i in (0..factors.len() - 1).rev() {
        compute_factor_local_suffixes(i);
    }
    compute_factor_local_suffixes(factors.len() - 1);

    local_suffixes.sort();
    println!("LOC SUFFXS: {:?}", local_suffixes);

    println!("MAP:");
    for local_suffix in local_suffixes {
        let item = map.get(local_suffix).unwrap();
        println!("{} -> {:?}", local_suffix, item);
    }
}
