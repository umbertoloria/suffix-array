pub fn get_custom_factors_and_more_using_chunk_size(
    icfl_indexes: &Vec<usize>,
    chunk_size: Option<usize>,
    str_length: usize,
) -> (Vec<usize>, Vec<bool>, Vec<usize>) {
    // From string "AAA|B|CAABCA|DCAABCA"
    //              ^   ^ ^      ^
    //        ICFL=[0,  3,4,     10]
    //  str_length = 17
    //  chunk_size = 3
    // Idx. 2 Is Custom  = [Char Index] => True if it's in the
    //                                     Last Custom Factor of
    //                                     an ICFL Factor.
    // Idx. 2 ICFL Fact. = [Char Index] => ICFL Factor Index of it
    // Returns:
    //   factor_indexes = [0, 3, 4, 7, 10, 11, 14]
    //                        (A,A,A,B,C,A,A,B,C,A,D,C,A,A,B,C,A)
    //   idx_to_is_custom   = [0,0,0,0,1,1,1,0,0,0,1,1,1,1,0,0,0]
    //   idx_to_icfl_factor = [0,0,0,1,2,2,2,2,2,2,3,3,3,3,3,3,3]
    let mut factor_indexes = Vec::new();
    let mut idx_to_is_custom = Vec::with_capacity(str_length);
    let mut idx_to_icfl_factor = Vec::with_capacity(str_length);

    if let Some(chunk_size) = chunk_size {
        for i in 0..icfl_indexes.len() {
            let curr_icfl_factor_index = icfl_indexes[i];

            // Curr ICFL Factor Size
            let curr_icfl_factor_size = if i < icfl_indexes.len() - 1 {
                icfl_indexes[i + 1]
            } else {
                str_length
            } - curr_icfl_factor_index;

            // Updating "factor_indexes"
            if curr_icfl_factor_size < chunk_size {
                // ICFL Factor can't be split.
                // For example: ICFL Factor "B".
                factor_indexes.push(curr_icfl_factor_index);
            } else {
                // ICFL Factor can be split.
                // For example: ICFL Factor "D|CAA|BCA".
                let smaller_cf_size = curr_icfl_factor_size % chunk_size;
                if smaller_cf_size > 0 {
                    // Here the first Custom Factor is smaller.
                    // For example: "D" of "D|CAA|BCA".
                    factor_indexes.push(curr_icfl_factor_index);
                }
                let mut curr_cf_idx = curr_icfl_factor_index + smaller_cf_size;
                while curr_cf_idx < curr_icfl_factor_index + curr_icfl_factor_size {
                    // Here all Custom Factor of size Chunk Size.
                    factor_indexes.push(curr_cf_idx);
                    curr_cf_idx += chunk_size;
                }
            }

            // Updating "idx_to_is_custom"
            let mut chars_left_in_icfl_factor = curr_icfl_factor_size;
            while chars_left_in_icfl_factor > chunk_size {
                idx_to_is_custom.push(true);
                chars_left_in_icfl_factor -= 1;
            }
            while chars_left_in_icfl_factor > 0 {
                idx_to_is_custom.push(false);
                chars_left_in_icfl_factor -= 1;
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
