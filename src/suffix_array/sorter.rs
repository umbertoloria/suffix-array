pub fn sort_pair_vector_of_indexed_strings(pair_vector: &mut Vec<(usize, &str)>) {
    pair_vector.sort_by(|a, b| {
        let a_string = a.1;
        let b_string = b.1;
        return a_string.cmp(b_string);
    });
}
