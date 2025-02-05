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
    src_length: usize,
) -> (Vec<usize>, Vec<bool>, Vec<usize>) {
    // From string "AAA|B|CAABCA|DCAABCA"
    // Es. ICFL=[0, 3, 4, 10]
    //  src_length = 17
    //  chunk_size = 3
    let mut custom_indexes = Vec::new();

    for i in 0..icfl_indexes.len() {
        let cur_factor_index = icfl_indexes[i];

        // Curr Factor Size
        let next_factor_index = if i < icfl_indexes.len() - 1 {
            icfl_indexes[i + 1]
        } else {
            src_length
        };
        let cur_factor_size = next_factor_index - cur_factor_index;

        // Es. on the 2nd factor "B": cur_factor_index=3, next_factor_index=4, cur_factor_size=1
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
            while cur_chunk_index < next_factor_index {
                custom_indexes.push(cur_chunk_index);
                cur_chunk_index += chunk_size;
            }
        }
    }

    // Custom Vec:  [Source Char Index] => True if it's part of the last Custom Factor of an
    //                                     ICFL Factor, so it's a Local Suffix of ICFL Factor.
    // Factor List: [Source Char Index] => ICFL Factor Index of that

    let mut is_custom_vec = Vec::with_capacity(src_length);
    let mut factor_list = Vec::with_capacity(src_length);

    for i in 0..icfl_indexes.len() {
        let cur_factor_index = icfl_indexes[i];

        // Curr Factor Size
        let next_factor_index = if i < icfl_indexes.len() - 1 {
            icfl_indexes[i + 1]
        } else {
            src_length
        };
        let cur_factor_size = next_factor_index - cur_factor_index;

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

        // Updating "factor_list"
        for _ in 0..cur_factor_size {
            factor_list.push(i);
        }
    }

    (
        //
        custom_indexes,
        is_custom_vec,
        factor_list,
    )
}

pub fn get_custom_factor_strings_from_custom_indexes(
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

pub fn get_max_size(factor_indexes: &Vec<usize>, src_length: usize) -> Option<usize> {
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
