use std::collections::HashSet;

pub fn get_kmers(src: &str, k: usize) -> HashSet<&str> {
    let mut kmers = HashSet::new();
    let mut offset = 0;
    while offset + k <= src.len() {
        let kmer = &src[offset..offset + k];
        kmers.insert(kmer);
        offset += 1;
    }
    kmers
}
