use crate::factorization::icfl::icfl;
use std::collections::HashMap;

pub fn main_suffix_array() {
    let src = "aaabcaabcadcaabca";
    let factors = icfl(src);
    println!("ICFL: {:?}", factors);

    let mut local_suffixes = Vec::new();
    for factor in &factors {
        for i in 0..factor.len() {
            let local_suffix = factor[i..factor.len()].to_string();
            if !local_suffixes.contains(&local_suffix) {
                local_suffixes.push(local_suffix);
                // println!("{:?}", local_suffix);
            }
        }
    }
    local_suffixes.sort();
    println!("LOC SUFFXS: {:?}", local_suffixes);

    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for local_suffix in &local_suffixes {
        println!("{}", local_suffix);

        // Factors: from second last to first
        for i in 1..factors.len() {
            let factor = &factors[factors.len() - 1 - i];
            if factor.len() >= local_suffix.len() {
                let left = factor.len() - local_suffix.len();
                let factor_suffix = &factor[left..factor.len()];
                if factor_suffix.eq(local_suffix) {
                    if !map.contains_key(local_suffix) {
                        map.insert(local_suffix.to_string(), vec![]);
                    }
                    map.get_mut(local_suffix).unwrap().push(75);
                    // println!(" ---> {factor_suffix} !!!");
                }
            }
        }

        // Factors: last
        let factor = &factors[factors.len() - 1];
        if factor.len() >= local_suffix.len() {
            let left = factor.len() - local_suffix.len();
            let factor_suffix = &factor[left..factor.len()];
            if factor_suffix.eq(local_suffix) {
                if !map.contains_key(local_suffix) {
                    map.insert(local_suffix.to_string(), vec![]);
                }
                map.get_mut(local_suffix).unwrap().push(75);
                // println!(" ---> {factor_suffix} !!!");
            }
        }
    }
    println!("MAP:\n");
    for local_suffix in &local_suffixes {
        let item = map.get(local_suffix).unwrap();
        println!("{} -> {:?}", local_suffix, item);
    }
}
