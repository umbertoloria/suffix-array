pub fn get_kmers(src: &str, k: usize) -> Vec<&str> {
    let mut kmers = Vec::new();
    let mut offset = 0;
    while offset + k <= src.len() {
        let kmer = &src[offset..offset + k];
        kmers.push(kmer);
        offset += 1;
    }
    kmers
}
