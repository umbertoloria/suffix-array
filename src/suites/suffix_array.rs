use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use std::collections::BTreeMap;

pub fn main_suffix_array() {
    let src = get_fasta_content("generated/000.fasta".into());
    let src_length = src.len();

    // Compute ICFL
    let factors = icfl(src.as_str());
    println!("{:?}", factors);

    let chunk_size = 5;
    println!("chunk_size={}", chunk_size);
    // TODO: Simplify algorithms by having string length as last item of these Factor Index vectors
    let icfl_indexes = get_indexes_from_factors(&factors);
    println!("ICFL_INDEXES={:?}", icfl_indexes);

    let custom_indexes = get_custom_factors(&icfl_indexes, chunk_size, src_length);
    println!("CSTM_INDEXES={:?}", custom_indexes);

    let max_size = get_max_size(&icfl_indexes, src_length).expect("max_size is not valid");
    let custom_max_size = get_max_size(&custom_indexes, src_length).expect("max_size is not valid");
    // println!("MAX_SIZE={:?}", max_size);
    // println!("CSTM_MAX_SIZE={:?}", custom_max_size);

    // TODO: Optimize both functions and rename them (and variables)
    let is_custom_vec = get_is_custom_vec(&icfl_indexes, src_length, chunk_size);
    // println!("is_custom_vec={:?}", is_custom_vec);

    let factor_list = get_factor_list(&icfl_indexes, src_length);
    // println!("factor_list={:?}", factor_list);

    let mut root = PrefixTrie {
        sons: BTreeMap::new(),
        rankings: Vec::new(),
    };
    for suffix_length in 1..custom_max_size + 1 {
        // Last factor
        let cur_factor_index = custom_indexes[custom_indexes.len() - 1];
        let cur_factor_length = src_length - cur_factor_index;
        if cur_factor_length >= suffix_length {
            let factor = &src[cur_factor_index..cur_factor_index + cur_factor_length];
            let suffix_index = cur_factor_length - suffix_length;
            let suffix = &factor[suffix_index..];
            let ranking = cur_factor_index + suffix_index;
            root.add_word(suffix.into(), ranking);
        }
        // All factors from first to second-last
        for j in 0..custom_indexes.len() - 1 {
            let cur_factor_index = custom_indexes[j];
            let cur_factor_length = custom_indexes[j + 1] - cur_factor_index;
            if cur_factor_length >= suffix_length {
                let factor = &src[cur_factor_index..cur_factor_index + cur_factor_length];
                let suffix_index = cur_factor_length - suffix_length;
                let suffix = &factor[suffix_index..];
                let ranking = cur_factor_index + suffix_index;
                root.add_word(suffix.into(), ranking);
            }
        }
    }
    root.print(0, "".into());

    /*
    // Local Suffixes and Rankings
    let ls_and_rankings =
        ls_and_rankings::get_local_suffixes_and_rankings_from_icfl_factors(&factors);
    for s_index in 0..ls_and_rankings.count {
        let (s, s_ranking) = ls_and_rankings.get_s_and_ranking_by_index(s_index);
        println!("{s} -> {s_ranking:?}");
    }

    // Creating Prefix Tree
    let prefix_tree = create_prefix_tree_from_ls_and_rankings(&ls_and_rankings);
    prefix_tree.show_tree(0);
    */
}
pub struct PrefixTrie {
    // TODO: Try to use HashMap but keeping chars sorted
    pub sons: BTreeMap<char, PrefixTrie>,
    pub rankings: Vec<usize>,
}
impl PrefixTrie {
    pub fn add_word(&mut self, word: String, cur_factor_index: usize) {
        if word.len() < 1 {
            return;
        }
        let (first_letter, rest) = separate_first_letter_from_rest(word.clone());
        if !self.sons.contains_key(&first_letter) {
            self.sons.insert(
                first_letter,
                PrefixTrie {
                    sons: BTreeMap::new(),
                    rankings: Vec::new(),
                },
            );
        }
        if rest.len() > 0 {
            self.sons
                .get_mut(&first_letter)
                .unwrap()
                .add_word(rest, cur_factor_index);
        } else {
            self.sons
                .get_mut(&first_letter)
                .unwrap()
                .rankings
                .push(cur_factor_index);
        }
    }
    pub fn print(&self, tabs_offset: usize, prefix: String) {
        /*if self.sons.len() == 1 {
            let char_key = self.sons.keys().collect::<Vec<_>>()[0];
            self.sons
                .get(char_key)
                .unwrap()
                .print(tabs_offset, format!("{}{}", prefix, char_key));
        } else {*/
        println!(
            "{}:{:2}: \"{}\" {:?}",
            " ".repeat(tabs_offset),
            tabs_offset,
            prefix,
            self.rankings
        );
        for (char_key, node) in &self.sons {
            node.print(tabs_offset + 1, format!("{}{}", prefix, char_key));
        }
        // }
    }
}
fn separate_first_letter_from_rest(str: String) -> (char, String) {
    let chars = str.chars();
    let first_letter = chars.collect::<Vec<_>>();
    let rest = (&str[1..]).to_string();
    (first_letter[0], rest)
}

fn get_indexes_from_factors(factors: &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 0;
    for factor in factors {
        result.push(i);
        i += factor.len();
    }
    result
}

fn get_custom_factors(icfl: &Vec<usize>, chunk_size: usize, src_length: usize) -> Vec<usize> {
    // From string "AAA|B|CAABCA|DCAABCA"
    // Es. ICFL=[0, 3, 4, 10]
    //  src_length = 17
    //  chunk_size = 3
    let mut result = Vec::new();
    for i in 0..icfl.len() {
        let cur_factor_index = icfl[i];
        let next_factor_index = if i < icfl.len() - 1 {
            icfl[i + 1]
        } else {
            src_length
        };
        let cur_factor_size = next_factor_index - cur_factor_index;
        // Es. on the 2nd factor "B": cur_factor_index=3, next_factor_index=4, cur_factor_size=1
        if cur_factor_size < chunk_size {
            // Es. on the 2nd factor "B": no space to perform chunking
            result.push(cur_factor_index);
        } else {
            let first_chunk_index_offset = cur_factor_size % chunk_size;
            if first_chunk_index_offset > 0 {
                // If factor was "DCAABCA" then we would have first_chunk_index_offset=1 (since
                // "cur_factor_size"=7 and "chunk_size"=3). So the index of this factor is not a
                // chunk, and it has to be added as factor "manually".
                result.push(cur_factor_index);
            } else {
                // If factor was "CAABCA" then we would have first_chunk_index_offset=0 (since
                // "cur_factor_size"=6 and "chunk_size"=3). So the index of this factor is also the
                // index of a chunk, so it'll be considered in the while statement below.
            }
            let mut cur_chunk_index = cur_factor_index + first_chunk_index_offset;
            while cur_chunk_index < next_factor_index {
                result.push(cur_chunk_index);
                cur_chunk_index += chunk_size;
            }
        }
    }
    // println!("ICFL_CUSTOM_FACTORS={:?}", res);
    result
}

pub fn get_is_custom_vec(
    icfl_indexes: &Vec<usize>,
    src_length: usize,
    chunk_size: usize,
) -> Vec<usize> {
    let mut result = Vec::with_capacity(src_length);
    for i in 0..src_length {
        result.push(
            if check_if_custom_index(icfl_indexes, src_length, i, chunk_size) {
                1
            } else {
                0
            },
        );
    }
    result
}
fn check_if_custom_index(
    icfl_indexes: &Vec<usize>,
    src_length: usize,
    index: usize,
    chunk_size: usize,
) -> bool {
    for i in 1..icfl_indexes.len() + 1 {
        let prev_factor_index = icfl_indexes[i - 1];
        let cur_factor_index = if i < icfl_indexes.len() {
            icfl_indexes[i]
        } else {
            src_length
        };
        if prev_factor_index <= index && index < cur_factor_index {
            if (cur_factor_index - index) <= chunk_size {
                return false;
            }
        }
    }
    true
}

fn get_factor_list(icfl_indexes: &Vec<usize>, src_length: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(src_length);
    for i in 0..src_length {
        result.push(get_factor(icfl_indexes, i));
    }
    result
}
fn get_factor(icfl_indexes: &Vec<usize>, index: usize) -> usize {
    for i in 0..icfl_indexes.len() - 1 {
        if icfl_indexes[i] <= index && index < icfl_indexes[i + 1] {
            return i;
        }
    }
    icfl_indexes.len() - 1
}

fn get_max_size(factor_indexes: &Vec<usize>, src_length: usize) -> Option<usize> {
    let mut result = None;
    for i in 0..factor_indexes.len() - 1 {
        let len = factor_indexes[i + 1] - factor_indexes[i];
        if let Some(result_value) = result {
            if result_value < len {
                result = Some(len);
            }
        } else {
            result = Some(len);
        }
    }
    let len = src_length - factor_indexes[factor_indexes.len() - 1];
    if let Some(result_value) = result {
        if result_value < len {
            result = Some(len);
        }
    } else {
        result = Some(len);
    }
    result
}
