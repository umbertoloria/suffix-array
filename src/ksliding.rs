use std::collections::HashSet;

// pub type KmersSet<'a> = BTreeSet<&'a str>;
pub type KmersSet<'a> = HashSet<&'a str>;

pub fn get_kmers(src: &str, k: usize) -> KmersSet {
    let mut kmers = KmersSet::new();
    let mut offset = 0;
    while offset + k <= src.len() {
        let kmer = &src[offset..offset + k];
        kmers.insert(kmer);
        offset += 1;
    }
    kmers
}
