pub fn get_indexes_from_factors(factors: &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 0;
    for factor in factors {
        result.push(i);
        i += factor.len();
    }
    result
}

pub fn get_custom_factors_and_more(
    icfl_indexes: &Vec<usize>,
    chunk_size: usize,
    str_length: usize,
) -> (Vec<usize>, Vec<bool>, Vec<usize>) {
    // From string "AAA|B|CAABCA|DCAABCA"
    // Es. ICFL=[0, 3, 4, 10]
    //  str_length = 17
    //  chunk_size = 3
    let mut custom_indexes = Vec::new();

    // Custom Vec:  [Source Char Index] => True if it's part of the last Custom Factor of an
    //                                     ICFL Factor, so it's a Local Suffix of ICFL Factor.
    // Factor List: [Source Char Index] => ICFL Factor Index of that
    let mut is_custom_vec = Vec::with_capacity(str_length);
    let mut icfl_factor_list = Vec::with_capacity(str_length);

    for i in 0..icfl_indexes.len() {
        let cur_factor_index = icfl_indexes[i];

        // Curr Factor Size
        let cur_factor_size = if i < icfl_indexes.len() - 1 {
            icfl_indexes[i + 1]
        } else {
            str_length
        } - cur_factor_index;

        // Updating "custom_indexes"
        // Es. on the 2nd factor "B": cur_factor_index=3, cur_factor_size=1
        if cur_factor_size < chunk_size {
            // Es. on the 2nd factor "B": no space to perform chunking
            custom_indexes.push(cur_factor_index);
        } else {
            let first_chunk_index_offset = cur_factor_size % chunk_size;
            if first_chunk_index_offset > 0 {
                // If factor was "DCAABCA" then we would have first_chunk_index_offset=1 (since
                // "cur_factor_size"=7 and "chunk_size"=3). So the index of this factor is not a
                // chunk, and it has to be added as factor "manually".
                custom_indexes.push(cur_factor_index);
            } else {
                // If factor was "CAABCA" then we would have first_chunk_index_offset=0 (since
                // "cur_factor_size"=6 and "chunk_size"=3). So the index of this factor is also the
                // index of a chunk, so it'll be considered in the while statement below.
            }
            let mut cur_chunk_index = cur_factor_index + first_chunk_index_offset;
            while cur_chunk_index < cur_factor_index + cur_factor_size {
                custom_indexes.push(cur_chunk_index);
                cur_chunk_index += chunk_size;
            }
        }

        // Updating "is_custom_vec"
        let mut remaining_chars_in_icfl_factor = cur_factor_size;
        if remaining_chars_in_icfl_factor >= chunk_size {
            while remaining_chars_in_icfl_factor > chunk_size {
                is_custom_vec.push(true);
                remaining_chars_in_icfl_factor -= 1;
            }
        }
        while remaining_chars_in_icfl_factor > 0 {
            is_custom_vec.push(false);
            remaining_chars_in_icfl_factor -= 1;
        }

        // Updating "icfl_factor_list"
        for _ in 0..cur_factor_size {
            icfl_factor_list.push(i);
        }
    }

    (
        //
        custom_indexes,
        is_custom_vec,
        icfl_factor_list,
    )
}
/*
pub fn get_icfl_factors_and_more_avoiding_custom_factorization(
    str_length: usize,
    icfl_indexes: &Vec<usize>,
) -> (Vec<usize>, Vec<bool>, Vec<usize>) {
    let mut custom_indexes = Vec::new();
    let mut is_custom_vec = Vec::new();
    let mut icfl_factor_list = Vec::new();

    for i in 0..icfl_indexes.len() {
        let cur_factor_index = icfl_indexes[i];

        // Curr Factor Size
        let cur_factor_size = if i < icfl_indexes.len() - 1 {
            icfl_indexes[i + 1]
        } else {
            str_length
        } - cur_factor_index;

        // Updating "custom_indexes"
        custom_indexes.push(cur_factor_index);

        // Updating "is_custom_vec"
        // Updating "icfl_factor_list"
        for _ in 0..cur_factor_size {
            is_custom_vec.push(false);
            icfl_factor_list.push(i);
        }
    }

    (custom_indexes, is_custom_vec, icfl_factor_list)
}
*/

pub fn get_max_factor_size(factor_indexes: &Vec<usize>, str_length: usize) -> usize {
    let mut result = factor_indexes[0];
    for i in 1..factor_indexes.len() - 1 {
        let curr_factor_size = factor_indexes[i + 1] - factor_indexes[i];
        if result < curr_factor_size {
            result = curr_factor_size;
        }
    }
    let curr_factor_size = str_length - factor_indexes[factor_indexes.len() - 1];
    if result < curr_factor_size {
        result = curr_factor_size;
    }
    result
}
