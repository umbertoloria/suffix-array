pub mod cfl;
pub mod custom_factorization;
pub mod icfl;
pub mod logging;

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
