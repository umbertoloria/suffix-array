use crate::factorization::icfl::icfl;
use crate::files::fasta::get_fasta_content;
use std::collections::BTreeMap;

pub fn main_suffix_array() {
    let src = get_fasta_content("generated/001.fasta".into());
    let src_str = src.as_str();
    let src_length = src.len();

    // Compute ICFL
    let factors = icfl(src_str);

    let chunk_size = 3;
    println!("chunk_size={}", chunk_size);
    // TODO: Simplify algorithms by having string length as last item of these Factor Index vectors
    let icfl_indexes = get_indexes_from_factors(&factors);
    println!("ICFL_INDEXES={:?}", icfl_indexes);
    println!("ICFL FACTORS: {:?}", factors);

    let custom_indexes = get_custom_factors(&icfl_indexes, chunk_size, src_length);
    let custom_indexes_len = custom_indexes.len();
    let custom_indexes_last_index = custom_indexes_len - 1;
    let custom_factors = get_custom_factor_strings_from_custom_indexes(src_str, &custom_indexes);
    println!("CSTM_INDEXES={:?}", custom_indexes);
    println!("CSTM FACTORS: {:?}", custom_factors);

    let max_size = get_max_size(&icfl_indexes, src_length).expect("max_size is not valid");
    let custom_max_size =
        get_max_size(&custom_indexes, src_length).expect("custom_max_size is not valid");
    // println!("MAX_SIZE={:?}", max_size);
    // println!("CSTM_MAX_SIZE={:?}", custom_max_size);

    // TODO: Optimize both functions and rename them (and variables)
    // Factor List: [Source Char Index] => True if it's part of the last Custom Factor of an
    //                                     ICFL Factor, so it's a Local Suffix
    let is_custom_vec = get_is_custom_vec(&icfl_indexes, src_length, chunk_size);
    println!("is_custom_vec={:?}", is_custom_vec);

    // Factor List: [Source Char Index] => ICFL Factor Index of that
    let factor_list = get_factor_list(&icfl_indexes, src_length);
    println!("factor_list={:?}", factor_list);

    let mut root = PrefixTrie {
        label: "\0".into(),
        sons: BTreeMap::new(),
        rankings_canonical: Vec::new(),
        rankings_custom: Vec::new(),
        rankings: Vec::new(),
        suffix_len: 0,
        min_father: None,
        max_father: None,
    };

    // Prefix Trie Structure create
    for curr_suffix_length in 1..custom_max_size + 1 {
        let mut ordered_list_of_custom_factor_local_suffix_index = Vec::new();
        // Last Custom Factor
        let curr_custom_factor_len = src_length - custom_indexes[custom_indexes_last_index];
        if curr_suffix_length <= curr_custom_factor_len {
            let custom_factor_local_suffix_index = src_length - curr_suffix_length;
            ordered_list_of_custom_factor_local_suffix_index.push(custom_factor_local_suffix_index);
        }
        // All Custom Factors from first to second-last
        for i_custom_factor in 0..custom_indexes_len - 1 {
            let curr_custom_factor_len =
                custom_indexes[i_custom_factor + 1] - custom_indexes[i_custom_factor];
            if curr_suffix_length <= curr_custom_factor_len {
                let custom_factor_local_suffix_index =
                    custom_indexes[i_custom_factor + 1] - curr_suffix_length;
                ordered_list_of_custom_factor_local_suffix_index
                    .push(custom_factor_local_suffix_index);
            }
        }

        // Filling "rankings_canonical" or "rankings_custom".
        for custom_factor_local_suffix_index in &ordered_list_of_custom_factor_local_suffix_index {
            // Implementation of "add_in_custom_prefix_trie".
            let custom_factor_local_suffix_index = *custom_factor_local_suffix_index;
            let suffix = &src[custom_factor_local_suffix_index
                ..custom_factor_local_suffix_index + curr_suffix_length];
            let chars_suffix = suffix.chars().collect::<Vec<_>>();

            let mut app_node = &mut root;

            let mut i_chars_of_suffix = 0;
            while i_chars_of_suffix < curr_suffix_length {
                let curr_letter = chars_suffix[i_chars_of_suffix];

                if !(*app_node).sons.contains_key(&curr_letter) {
                    (*app_node).sons.insert(
                        curr_letter,
                        PrefixTrie {
                            label: format!("{}{}", app_node.label, curr_letter),
                            sons: BTreeMap::new(),
                            rankings_canonical: Vec::new(),
                            rankings_custom: Vec::new(),
                            rankings: Vec::new(),
                            suffix_len: i_chars_of_suffix + 1,
                            min_father: None,
                            max_father: None,
                        },
                    );
                }
                app_node = app_node.sons.get_mut(&curr_letter).unwrap();

                i_chars_of_suffix += 1;
            }
            // TODO: Here we could create an interesting wrapping among real "non-bridge" nodes
            if is_custom_vec[custom_factor_local_suffix_index] {
                app_node
                    .rankings_custom
                    .push(custom_factor_local_suffix_index);
            } else {
                app_node
                    .rankings_canonical
                    .push(custom_factor_local_suffix_index);
            }
        }
    }

    // Ordering rankings.
    /*
    println!("Before merge");
    root.print(0, "".into());
    */
    root.merge_rankings_and_sort_recursive(src_str, src_length);
    // println!("Before in_prefix");
    // root.print(0, "".into());

    // In Prefix Merge: bit vector
    root.in_prefix_merge_bit_vector(src_str, &icfl_indexes, &is_custom_vec, &factor_list);
    root.print(0, "".into());
}

fn get_custom_factor_strings_from_custom_indexes(
    src: &str,
    custom_indexes: &Vec<usize>,
) -> Vec<String> {
    let mut result = Vec::with_capacity(custom_indexes.len());
    let mut i = 0;
    while i < custom_indexes.len() - 1 {
        let cur_factor_index = custom_indexes[i];
        let next_factor_index = custom_indexes[i + 1];
        let slice = &src[cur_factor_index..next_factor_index];
        result.push(slice.into());
        i += 1;
    }
    let cur_factor_index = custom_indexes[i];
    let next_factor_index = src.len();
    let slice = &src[cur_factor_index..next_factor_index];
    result.push(slice.into());
    result
}

// Prefix Trie
pub struct PrefixTrie {
    pub label: String,
    // TODO: Try to use HashMap but keeping chars sorted
    pub sons: BTreeMap<char, PrefixTrie>,
    pub rankings_canonical: Vec<usize>,
    pub rankings_custom: Vec<usize>,
    pub rankings: Vec<usize>,
    pub suffix_len: usize,
    pub min_father: Option<usize>,
    pub max_father: Option<usize>,
}
impl PrefixTrie {
    pub fn merge_rankings_and_sort_recursive(&mut self, src: &str, src_length: usize) {
        // Single "rankings" list
        for local_suffix_index in &self.rankings_canonical {
            self.rankings.push(*local_suffix_index);
        }
        for local_suffix_index in &self.rankings_custom {
            self.rankings.push(*local_suffix_index);
        }

        if self.rankings.len() > 1 {
            // Sort global suffixes
            struct StringAndIndex<'a> {
                global_suffix: &'a str,
                index: usize,
            }
            let mut list = Vec::new();
            for ranking in &self.rankings {
                let ranking = *ranking;
                let global_suffix = &src[ranking..src_length];
                list.push(StringAndIndex {
                    global_suffix,
                    index: ranking,
                });
            }

            // TODO: Maybe sorting is sometimes avoidable
            list.sort_by(|a, b| a.global_suffix.cmp(b.global_suffix));

            // Update list only if strings were actually sorted and moved.
            self.rankings.clear();
            for item in &list {
                self.rankings.push(item.index);
            }
        }

        // Recursive calls...
        for (_, children) in &mut self.sons {
            children.merge_rankings_and_sort_recursive(src, src_length);
        }
    }
    pub fn in_prefix_merge_bit_vector(
        &mut self,
        src: &str,
        icfl_indexes: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) {
        let father_rankings = &self.rankings;

        for (_, child) in &mut self.sons {
            let mut list = Vec::new();

            let child_rankings = &mut child.rankings;
            if child_rankings.is_empty() {
                // TODO: Should treat this node like a "bridge", skip for now
                break;
            }

            let child_offset = child.suffix_len;
            let mut min_father = None;

            for i in 0..father_rankings.len() {
                let cmp1_from_child = &src[child_rankings[0]..child_rankings[0] + child_offset];
                let cmp2_from_father = &src
                    [father_rankings[i]..usize::min(father_rankings[i] + child_offset, src.len())];
                if cmp1_from_child <= cmp2_from_father {
                    min_father = Some(i);
                    break;
                }
            }
            if min_father.is_none() {
                list = child_rankings.clone();
                continue;
            }
            let min_father = min_father.unwrap();

            let cmp1_from_child = &src[child_rankings[0]..child_rankings[0] + child_offset];
            let cmp2_from_father =
                &src[father_rankings[min_father]..father_rankings[min_father] + child_offset];
            if cmp1_from_child < cmp2_from_father {
                child.min_father = Some(min_father);
                list = child_rankings.clone();
                continue;
            }

            let mut i = min_father;
            while i < father_rankings.len() {
                let cmp1_from_child = &src[child_rankings[0]..child_rankings[0] + child_offset];
                let cmp2_from_father = &src[father_rankings[i]..father_rankings[i] + child_offset];
                if cmp1_from_child != cmp2_from_father {
                    break;
                }
                i += 1;
            }
            let max_father = i;

            child.min_father = Some(min_father);
            child.max_father = Some(max_father);

            // TODO: Pre-allocate "list"

            let mut i = min_father;
            let mut j = 0;
            while i < max_father && j < child_rankings.len() {
                let x = father_rankings[i];
                let y = child_rankings[j];
                let result_rules = Self::rules(
                    x,
                    y,
                    child_offset,
                    src,
                    &icfl_indexes,
                    &is_custom_vec,
                    &factor_list,
                );
                if !result_rules {
                    list.push(father_rankings[i]);
                    i += 1;
                } else {
                    list.push(child_rankings[j]);
                    j += 1;
                }
            }
            while j < child_rankings.len() {
                list.push(child_rankings[j]);
                j += 1;
            }
            while i < max_father {
                list.push(father_rankings[i]);
                i += 1;
            }
            child_rankings.clear();
            child_rankings.append(&mut list);
        }

        // Recursive calls...
        for (_, child) in &mut self.sons {
            child.in_prefix_merge_bit_vector(src, icfl_indexes, is_custom_vec, factor_list);
        }
    }
    fn rules(
        x: usize,
        y: usize,
        child_offset: usize,
        src: &str,
        icfl_list: &Vec<usize>,
        is_custom_vec: &Vec<bool>,
        factor_list: &Vec<usize>,
    ) -> bool {
        let icfl_list_size = icfl_list.len();
        if is_custom_vec[x] && is_custom_vec[y] {
            let cmp1 = &src[y + child_offset..];
            let cmp2 = &src[x + child_offset..];
            if cmp1 < cmp2 {
                true
            } else {
                false
            }
        } else if is_custom_vec[x] {
            if factor_list[x] <= factor_list[y] {
                if y >= icfl_list[icfl_list_size - 1] {
                    true
                } else {
                    false
                }
            } else {
                let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }
            }
        } else if is_custom_vec[y] {
            if factor_list[y] <= factor_list[x] {
                if x >= icfl_list[icfl_list_size - 1] {
                    false
                } else {
                    true
                }
            } else {
                let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }
            }
        } else if x >= icfl_list[icfl_list_size - 1] && y >= icfl_list[icfl_list_size - 1] {
            false
        } else if factor_list[x] == factor_list[y] {
            true
        } else {
            if x >= icfl_list[icfl_list_size - 1] {
                false
            } else if y >= icfl_list[icfl_list_size - 1] {
                let cmp1 = &src[y + child_offset..];
                let cmp2 = &src[x + child_offset..];
                if cmp1 < cmp2 {
                    true
                } else {
                    false
                }
            } else {
                if x > y {
                    true
                } else {
                    let cmp1 = &src[y + child_offset..];
                    let cmp2 = &src[x + child_offset..];
                    if cmp1 < cmp2 {
                        true
                    } else {
                        false
                    }
                }
            }
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
            "{}|{:2}: \"{}\" {}, min={}, max={}",
            " ".repeat(tabs_offset),
            tabs_offset,
            prefix,
            // self.label,
            if self.rankings.is_empty() && self.suffix_len > 0 {
                format!("{:?} {:?}", self.rankings_canonical, self.rankings_custom)
            } else {
                format!("{:?}", self.rankings)
            },
            if let Some(min_father) = self.min_father {
                min_father as i16
            } else {
                -1
            },
            if let Some(max_father) = self.max_father {
                max_father as i16
            } else {
                -1
            },
        );
        for (char_key, node) in &self.sons {
            node.print(tabs_offset + 1, format!("{}{}", prefix, char_key));
        }
        // }
    }
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
) -> Vec<bool> {
    let mut result = Vec::with_capacity(src_length);
    for i in 0..src_length {
        result.push(check_if_custom_index(
            icfl_indexes,
            src_length,
            i,
            chunk_size,
        ));
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
