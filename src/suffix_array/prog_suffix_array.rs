use std::collections::HashMap;

// let mut wbsa = (0..src_length).collect::<Vec<_>>();
// let mut wbsa_indexes = HashMap::new();
pub type ProgSuffixArrayIndexes = HashMap<usize, (usize, usize)>;
pub struct ProgSuffixArray {
    wbsa: Vec<usize>,
    wbsa_indexes: ProgSuffixArrayIndexes,
}
impl ProgSuffixArray {
    pub fn new(str_length: usize) -> Self {
        Self {
            wbsa: (0..str_length).collect::<Vec<_>>(),
            wbsa_indexes: HashMap::new(),
        }
    }
}
