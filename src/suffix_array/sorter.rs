pub fn sort_pair_vector_of_indexed_strings(pair_vector: &mut Vec<(usize, &str)>) {
    pair_vector.sort_by(|a, b| a.1.cmp(b.1));
}
