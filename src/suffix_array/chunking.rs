pub fn get_indexes_from_factors(factors: &Vec<String>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut i = 0;
    for factor in factors {
        result.push(i);
        i += factor.len();
    }
    result
}

pub fn get_custom_factors_and_more_using_chunk_size(
    icfl_indexes: &Vec<usize>,
    chunk_size: Option<usize>,
    str_length: usize,
) -> (Vec<usize>, Vec<bool>, Vec<usize>) {
    // From string "AAA|B|CAABCA|DCAABCA"
    // Es. ICFL=[0, 3, 4, 10]
    //  str_length = 17
    //  chunk_size = 3

    // Custom Vec:  [Source Char Index] => True if it's part of the last Custom Factor of an
    //                                     ICFL Factor, so it's a Local Suffix of ICFL Factor.
    // Factor List: [Source Char Index] => ICFL Factor Index of that
    let mut factor_indexes = Vec::new();
    let mut idx_to_is_custom = Vec::with_capacity(str_length);
    let mut idx_to_icfl_factor = Vec::with_capacity(str_length);

    if let Some(chunk_size) = chunk_size {
        for i in 0..icfl_indexes.len() {
            let curr_icfl_factor_index = icfl_indexes[i];

            // Curr  ICFL Factor Size
            let curr_icfl_factor_size = if i < icfl_indexes.len() - 1 {
                icfl_indexes[i + 1]
            } else {
                str_length
            } - curr_icfl_factor_index;

            // Updating "factor_indexes"
            // Es. on the 2nd factor "B": curr_icfl_factor_index=3, curr_icfl_factor_size=1
            if curr_icfl_factor_size < chunk_size {
                // Es. on the 2nd factor "B": no space to perform chunking
                factor_indexes.push(curr_icfl_factor_index);
            } else {
                let first_chunk_index_offset = curr_icfl_factor_size % chunk_size;
                if first_chunk_index_offset > 0 {
                    // If factor was "DCAABCA" then we would have first_chunk_index_offset=1 (since
                    // "cur_factor_size"=7 and "chunk_size"=3). So the index of this factor is not a
                    // chunk, and it has to be added as factor "manually".
                    factor_indexes.push(curr_icfl_factor_index);
                } else {
                    // If factor was "CAABCA" then we would have first_chunk_index_offset=0 (since
                    // "cur_factor_size"=6 and "chunk_size"=3). So the index of this factor is also the
                    // index of a chunk, so it'll be considered in the while statement below.
                }
                let mut cur_chunk_index = curr_icfl_factor_index + first_chunk_index_offset;
                while cur_chunk_index < curr_icfl_factor_index + curr_icfl_factor_size {
                    factor_indexes.push(cur_chunk_index);
                    cur_chunk_index += chunk_size;
                }
            }

            // Updating "idx_to_is_custom"
            let mut remaining_chars_in_icfl_factor = curr_icfl_factor_size;
            while remaining_chars_in_icfl_factor > chunk_size {
                idx_to_is_custom.push(true);
                remaining_chars_in_icfl_factor -= 1;
            }
            while remaining_chars_in_icfl_factor > 0 {
                idx_to_is_custom.push(false);
                remaining_chars_in_icfl_factor -= 1;
            }

            // Updating "idx_to_icfl_factor"
            for _ in 0..curr_icfl_factor_size {
                idx_to_icfl_factor.push(i);
            }
        }
    } else {
        for i in 0..icfl_indexes.len() {
            let curr_icfl_factor_index = icfl_indexes[i];

            // Curr ICFL Factor Size
            let curr_icfl_factor_size = if i < icfl_indexes.len() - 1 {
                icfl_indexes[i + 1]
            } else {
                str_length
            } - curr_icfl_factor_index;

            // Updating "factor_indexes"
            factor_indexes.push(curr_icfl_factor_index);

            // Updating "idx_to_is_custom" and "idx_to_icfl_factor"
            for _ in 0..curr_icfl_factor_size {
                idx_to_is_custom.push(false);
                idx_to_icfl_factor.push(i);
            }
        }
    }

    (
        //
        factor_indexes,
        idx_to_is_custom,
        idx_to_icfl_factor,
    )
}

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
